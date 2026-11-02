use axum::{extract::{Path, Query, State}, http::{HeaderMap, HeaderValue, Method, StatusCode}, response::{IntoResponse, Response}, routing::{get, post}, Json, Router};
use chrono::{Duration, Utc};
use futures::TryStreamExt;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};
use mongodb::{bson::{doc, oid::ObjectId, DateTime, Document}, options::IndexOptions, Client, IndexModel};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashMap, env, sync::Arc};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

// Optional SMTP mailer. When unset, the API runs in development mode and
// returns the OTP in the response instead of emailing it.
#[derive(Clone)]
struct Mailer { transport: AsyncSmtpTransport<Tokio1Executor>, from: String }

#[derive(Clone)]
struct AppState {
    db: mongodb::Database,
    jwt: String,
    mailer: Option<Arc<Mailer>>,
    admin_token: Option<String>,
    app_url: String,
}

#[derive(Debug, thiserror::Error)]
enum ApiError {
    #[error("{0}")] Bad(String),
    #[error("Authentication required")] Unauthorized,
    #[error("This action is not available")] Forbidden,
    #[error("Could not send the verification email. Try again shortly.")] Mail,
    #[error("{0}")] Db(#[from] mongodb::error::Error),
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Bad(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Mail => StatusCode::BAD_GATEWAY,
            Self::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({"error": self.to_string()}))).into_response()
    }
}

#[derive(Serialize, Deserialize)] struct Claims { sub: String, name: String, admin: bool, exp: usize }
#[derive(Deserialize)] struct OtpRequest { email: String }
#[derive(Deserialize)] struct VerifyRequest { email: String, code: String, display_name: String }
#[derive(Serialize, Deserialize, Clone)] struct Otp { email: String, code: String, expires_at: DateTime }
#[derive(Serialize, Deserialize, Clone)] struct Listing { #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] id: Option<ObjectId>, kind: String, title: String, description: String, price: f64, area: String, owner_email: String, owner_name: String, status: String, meta: HashMap<String, String>, created_at: DateTime, expires_at: DateTime }
#[derive(Deserialize)] struct NewListing { kind: String, title: String, description: String, price: f64, area: String, #[serde(default)] meta: HashMap<String, String> }
#[derive(Deserialize)] struct ListingQuery { kind: Option<String>, search: Option<String> }
#[derive(Deserialize)] struct ReportRequest { listing_id: String, reason: String }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().with_env_filter("campus_circle_api=debug,tower_http=debug").init();

    let uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(uri).await?;
    let db = client.database(&env::var("MONGODB_DATABASE").unwrap_or_else(|_| "campus_circle".into()));
    create_indexes(&db).await?;

    let jwt = env::var("JWT_SECRET").unwrap_or_else(|_| "development-secret-change-me".into());
    if jwt == "development-secret-change-me" {
        tracing::warn!("JWT_SECRET is unset; using the insecure development default. Set JWT_SECRET in production.");
    }

    let mailer = build_mailer();
    if mailer.is_none() {
        tracing::warn!("SMTP is not configured; running in development OTP mode (codes are returned in the API response). Configure SMTP_* before allowing real users.");
    }
    let admin_token = env::var("ADMIN_TOKEN").ok().filter(|v| !v.is_empty());
    let app_url = env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".into());

    let state = AppState { db, jwt, mailer, admin_token, app_url };

    // Lock CORS to the configured origin. With credentials/Authorization headers
    // a wildcard origin is both unsafe and disallowed by browsers.
    let origin = env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".into());
    let cors = CorsLayer::new()
        .allow_origin(origin.parse::<HeaderValue>().expect("CORS_ORIGIN must be a valid origin URL"))
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]);

    let app = Router::new()
        .route("/health", get(|| async { Json(json!({"status": "ok"})) }))
        .route("/auth/request-otp", post(request_otp))
        .route("/auth/verify-otp", post(verify_otp))
        .route("/listings", get(listings).post(create_listing))
        .route("/listings/{id}/contact-request", post(contact_request))
        .route("/reports", post(report))
        .route("/dev/seed", post(seed))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = env::var("API_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on {addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

fn build_mailer() -> Option<Arc<Mailer>> {
    let host = env::var("SMTP_HOST").ok().filter(|v| !v.is_empty())?;
    let user = env::var("SMTP_USERNAME").ok().filter(|v| !v.is_empty())?;
    let pass = env::var("SMTP_PASSWORD").ok().filter(|v| !v.is_empty())?;
    let from = env::var("SMTP_FROM").unwrap_or_else(|_| user.clone());
    let port: u16 = env::var("SMTP_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(587);
    let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)
        .ok()?
        .port(port)
        .credentials(Credentials::new(user, pass))
        .build();
    Some(Arc::new(Mailer { transport, from }))
}

async fn create_indexes(db: &mongodb::Database) -> mongodb::error::Result<()> {
    let unique = IndexOptions::builder().unique(true).build();
    db.collection::<Value>("users").create_index(IndexModel::builder().keys(doc! {"email": 1}).options(unique).build()).await?;
    db.collection::<Listing>("listings").create_index(IndexModel::builder().keys(doc! {"kind": 1, "created_at": -1}).build()).await?;
    // One pending OTP per email; the TTL index expires the document at expires_at.
    let otp_unique = IndexOptions::builder().unique(true).build();
    db.collection::<Otp>("otps").create_index(IndexModel::builder().keys(doc! {"email": 1}).options(otp_unique).build()).await?;
    let ttl = IndexOptions::builder().expire_after(std::time::Duration::from_secs(0)).build();
    db.collection::<Otp>("otps").create_index(IndexModel::builder().keys(doc! {"expires_at": 1}).options(ttl).build()).await?;
    Ok(())
}

fn valid_email(email: &str) -> bool {
    email.len() > 12 && email.to_ascii_lowercase().ends_with("@purdue.edu") && email.split('@').next().is_some_and(|s| !s.is_empty())
}

async fn send_otp_email(mailer: &Mailer, app_url: &str, to: &str, code: &str) -> Result<(), ApiError> {
    let body = format!(
        "Your Campus Circle verification code is {code}.\n\nIt expires in 10 minutes. Enter it at {app_url} to finish signing in.\n\nIf you did not request this, you can ignore this email.\n\nCampus Circle is student-run and not affiliated with or endorsed by Purdue University."
    );
    let message = Message::builder()
        .from(mailer.from.parse().map_err(|_| ApiError::Mail)?)
        .to(to.parse().map_err(|_| ApiError::Mail)?)
        .subject("Your Campus Circle verification code")
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .map_err(|_| ApiError::Mail)?;
    mailer.transport.send(message).await.map_err(|e| { tracing::error!("smtp send failed: {e}"); ApiError::Mail })?;
    Ok(())
}

async fn request_otp(State(s): State<AppState>, Json(input): Json<OtpRequest>) -> Result<Json<Value>, ApiError> {
    let email = input.email.trim().to_ascii_lowercase();
    if !valid_email(&email) { return Err(ApiError::Bad("Use a valid @purdue.edu email".into())); }
    let code = format!("{:06}", rand::rng().random_range(0..1_000_000));
    let expires_at = DateTime::from_millis((Utc::now() + Duration::minutes(10)).timestamp_millis());
    // Upsert so a re-request replaces the prior code for this email.
    s.db.collection::<Otp>("otps")
        .update_one(doc! {"email": &email}, doc! {"$set": {"code": &code, "expires_at": expires_at}})
        .upsert(true)
        .await?;

    match &s.mailer {
        Some(mailer) => {
            send_otp_email(mailer, &s.app_url, &email, &code).await?;
            Ok(Json(json!({"message": "Check your Purdue email for the code."})))
        }
        // Development mode only: no SMTP configured, so surface the code.
        None => Ok(Json(json!({"message": "Code generated (development mode)", "development_code": code}))),
    }
}

async fn verify_otp(State(s): State<AppState>, Json(input): Json<VerifyRequest>) -> Result<Json<Value>, ApiError> {
    let email = input.email.trim().to_ascii_lowercase();
    let record = s.db.collection::<Otp>("otps").find_one(doc! {"email": &email}).await?;
    let now = Utc::now().timestamp_millis();
    let ok = record.as_ref().is_some_and(|o| o.code == input.code && o.expires_at.timestamp_millis() > now);
    if !ok { return Err(ApiError::Bad("That code is invalid or expired".into())); }
    s.db.collection::<Otp>("otps").delete_one(doc! {"email": &email}).await?;

    let admin = email.starts_with("admin+");
    s.db.collection::<Value>("users")
        .update_one(doc! {"email": &email}, doc! {"$set": {"display_name": &input.display_name, "verified": true, "verified_at": DateTime::now()}, "$setOnInsert": {"created_at": DateTime::now()}})
        .upsert(true)
        .await?;
    let claims = Claims { sub: email.clone(), name: input.display_name.clone(), admin, exp: (Utc::now() + Duration::days(7)).timestamp() as usize };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(s.jwt.as_bytes())).map_err(|_| ApiError::Unauthorized)?;
    Ok(Json(json!({"token": token, "email": email, "display_name": input.display_name, "admin": admin})))
}

fn auth(headers: &HeaderMap, s: &AppState) -> Result<Claims, ApiError> {
    let raw = headers.get("authorization").and_then(|v| v.to_str().ok()).and_then(|v| v.strip_prefix("Bearer ")).ok_or(ApiError::Unauthorized)?;
    decode::<Claims>(raw, &DecodingKey::from_secret(s.jwt.as_bytes()), &Validation::default()).map(|v| v.claims).map_err(|_| ApiError::Unauthorized)
}

fn public_listing(l: Listing) -> Value {
    json!({"id": l.id.map(|v| v.to_hex()).unwrap_or_default(), "kind": l.kind, "title": l.title, "description": l.description, "price": l.price, "area": l.area, "owner_name": l.owner_name, "status": l.status, "meta": l.meta, "created_at": l.created_at.to_string()})
}

async fn listings(State(s): State<AppState>, Query(q): Query<ListingQuery>) -> Result<Json<Value>, ApiError> {
    let mut filter = doc! {"status": "active"};
    if let Some(k) = q.kind.filter(|v| !v.is_empty()) { filter.insert("kind", k); }
    if let Some(search) = q.search.filter(|v| !v.is_empty()) {
        filter.insert("$or", vec![doc! {"title": {"$regex": &search, "$options": "i"}}, doc! {"description": {"$regex": &search, "$options": "i"}}, doc! {"area": {"$regex": &search, "$options": "i"}}]);
    }
    let rows = s.db.collection::<Listing>("listings").find(filter).sort(doc! {"created_at": -1}).limit(100).await?.try_collect::<Vec<_>>().await?;
    Ok(Json(Value::Array(rows.into_iter().map(public_listing).collect())))
}

async fn create_listing(State(s): State<AppState>, headers: HeaderMap, Json(input): Json<NewListing>) -> Result<Json<Value>, ApiError> {
    let user = auth(&headers, &s)?;
    if !["housing", "marketplace", "ride"].contains(&input.kind.as_str()) || input.title.len() < 5 || input.description.len() < 20 {
        return Err(ApiError::Bad("Add a clear title and at least 20 characters of detail".into()));
    }
    let mut listing = Listing { id: None, kind: input.kind, title: input.title, description: input.description, price: input.price, area: input.area, owner_email: user.sub, owner_name: user.name, status: "active".into(), meta: input.meta, created_at: DateTime::now(), expires_at: DateTime::from_millis((Utc::now() + Duration::days(30)).timestamp_millis()) };
    let result = s.db.collection::<Listing>("listings").insert_one(&listing).await?;
    listing.id = result.inserted_id.as_object_id();
    Ok(Json(public_listing(listing)))
}

async fn contact_request(State(s): State<AppState>, headers: HeaderMap, Path(id): Path<String>) -> Result<Json<Value>, ApiError> {
    let user = auth(&headers, &s)?;
    let oid = ObjectId::parse_str(id).map_err(|_| ApiError::Bad("Invalid listing".into()))?;
    s.db.collection::<Document>("contact_requests").insert_one(doc! {"listing_id": oid, "requester_email": user.sub, "status": "pending", "created_at": DateTime::now()}).await?;
    Ok(Json(json!({"message": "Contact request sent. The lister can approve it in-app."})))
}

async fn report(State(s): State<AppState>, headers: HeaderMap, Json(input): Json<ReportRequest>) -> Result<Json<Value>, ApiError> {
    let user = auth(&headers, &s)?;
    let oid = ObjectId::parse_str(input.listing_id).map_err(|_| ApiError::Bad("Invalid listing".into()))?;
    s.db.collection::<Document>("reports").insert_one(doc! {"listing_id": oid, "reporter_email": user.sub, "reason": input.reason, "status": "open", "created_at": DateTime::now()}).await?;
    Ok(Json(json!({"message": "Report sent for review"})))
}

// Seeding is destructive-ish (writes demo data) and must never be open in
// production. It is disabled unless ADMIN_TOKEN is set, and then requires a
// matching `X-Admin-Token` header.
async fn seed(State(s): State<AppState>, headers: HeaderMap) -> Result<Json<Value>, ApiError> {
    let expected = s.admin_token.as_ref().ok_or(ApiError::Forbidden)?;
    let provided = headers.get("x-admin-token").and_then(|v| v.to_str().ok()).unwrap_or_default();
    if provided != expected { return Err(ApiError::Forbidden); }

    let c = s.db.collection::<Listing>("listings");
    if c.count_documents(doc! {}).await? > 0 { return Ok(Json(json!({"inserted": 0}))); }
    let now = DateTime::now();
    let exp = DateTime::from_millis((Utc::now() + Duration::days(30)).timestamp_millis());
    let data = vec![
        ("housing", "Sunny room near Chauncey", 720.0, "Chauncey Hill", "Furnished room with a flexible May start.", [("dates", "May–August"), ("room", "Private room")]),
        ("marketplace", "Trek commuter bike", 180.0, "Central campus", "Recently tuned, with lights and a U-lock.", [("condition", "Good"), ("category", "Bikes")]),
        ("ride", "West Lafayette → Chicago", 24.0, "PMU pickup", "Leaving Friday after class with room for one carry-on.", [("departure", "Fri · 4:30 PM"), ("seats", "2 seats")]),
    ];
    let docs = data.into_iter().map(|(kind, title, price, area, description, meta)| Listing { id: None, kind: kind.into(), title: title.into(), description: description.into(), price, area: area.into(), owner_email: "demo@purdue.edu".into(), owner_name: "Verified student".into(), status: "active".into(), meta: meta.into_iter().map(|(k, v)| (k.into(), v.into())).collect(), created_at: now, expires_at: exp }).collect::<Vec<_>>();
    let n = c.insert_many(docs).await?.inserted_ids.len();
    Ok(Json(json!({"inserted": n})))
}

# Campus Circle

**Live:** <https://cc.siv19.dev>

Campus Circle is a student-run, privacy-first exchange for verified Purdue students: housing, marketplace listings, and rides in one responsive app. It was started after the founder personally experienced a scam and wanted student interactions to begin with stronger verification and safer defaults.

> Unofficial project. Campus Circle is not affiliated with, endorsed by, or operated by Purdue University.

## Monorepo

- `apps/web` — Next.js 16, React 19, TypeScript (static export → Cloudflare Pages)
- `apps/api` — Rust, Axum, official MongoDB driver (`Dockerfile`, `fly.toml`)
- `docker-compose.yml` — MongoDB 8 and the Rust API

## Run locally

Requirements: Node.js 20+ and Docker Desktop.

```bash
npm install
docker compose up -d --build
curl -X POST http://localhost:8081/dev/seed -H "X-Admin-Token: dev-seed-token"
npm run dev
```

Open [http://localhost:3000](http://localhost:3000). With no SMTP configured, the API runs in **development OTP mode**: the verification code is returned in the `request-otp` response and shown in the dialog. Configure `SMTP_*` to send real emails (and the code stops being returned).

## Environment

Copy `.env.example` to `.env` for the API and `apps/web/.env.local` for the client. See `.env.example` for every variable. Key ones:

- `MONGODB_URI` / `MONGODB_DATABASE` — connection (local or Atlas SRV URI).
- `JWT_SECRET` — session signing key; use `openssl rand -hex 32` in production.
- `CORS_ORIGIN` / `APP_URL` — the web origin allowed to call the API and linked in emails.
- `SMTP_HOST` / `SMTP_PORT` / `SMTP_USERNAME` / `SMTP_PASSWORD` / `SMTP_FROM` — OTP email delivery.
- `ADMIN_TOKEN` — enables `POST /dev/seed`; the request must send a matching `X-Admin-Token` header. Leave unset to disable seeding.
- `NEXT_PUBLIC_API_URL` — the web client's API base; **baked in at build time**.

## Implemented MVP

- Mobile-first responsive home and listing board
- Purdue email OTP verification and signed sessions
- Housing, marketplace, and ride categories with search/filtering
- Authenticated listing creation
- Consent-based contact requests
- In-app report flow and MongoDB moderation records
- MongoDB indexes, expiry fields, seeded demo data, and health endpoint
- Safety guidance and founder motivation without fabricated claims

## Deployment

The two apps deploy separately. Cloudflare serves the static frontend; the
Rust API needs a container host (it is a long-running Axum process using the
native MongoDB driver, which Cloudflare Workers cannot run).

```
cc.siv19.dev       → Cloudflare Pages   (apps/web, static export)
api.cc.siv19.dev   → Fly.io / Railway / Render container (apps/api)
MongoDB Atlas      → managed MongoDB via MONGODB_URI
```

### 1. MongoDB Atlas

Create a free M0 cluster and a database user, allow your API host's egress IPs
under Network Access, and copy the SRV connection string into `MONGODB_URI`.
The API creates its own indexes (including a TTL index on OTPs) on boot.

### 2. API on Fly.io

`apps/api` ships a `Dockerfile` and `fly.toml`.

```bash
cd apps/api
fly launch --no-deploy          # reuses the committed fly.toml
fly secrets set \
  MONGODB_URI="mongodb+srv://USER:PASS@cluster0.xxxx.mongodb.net/?retryWrites=true&w=majority" \
  JWT_SECRET="$(openssl rand -hex 32)" \
  ADMIN_TOKEN="$(openssl rand -hex 16)" \
  SMTP_HOST="smtp.zeptomail.com" SMTP_PORT="587" \
  SMTP_USERNAME="emailapikey" SMTP_PASSWORD="your-smtp-token" \
  SMTP_FROM="no-reply@cc.siv19.dev"
fly deploy
fly certs add api.cc.siv19.dev
```

Verify: `curl https://api.cc.siv19.dev/health` → `{"status":"ok"}`.
Railway and Render work the same way: deploy `apps/api` from the repo, set the
same env vars, attach the `api.cc.siv19.dev` domain.

### 3. Frontend on Cloudflare Pages

`apps/web` is configured for static export (`output: "export"`), so it deploys
as plain static assets with no adapter.

In Cloudflare → Workers & Pages → Pages → connect `sivab193/campus-circle`:

- **Root directory:** `apps/web`
- **Build command:** `npm install && npm run build`
- **Build output directory:** `out`
- **Environment variable:** `NEXT_PUBLIC_API_URL = https://api.cc.siv19.dev`

Then add the custom domain `cc.siv19.dev` under the project's Custom domains
tab. Because `NEXT_PUBLIC_API_URL` is inlined at build time, redeploy after
changing it.

## Production notes

The API enforces `CORS_ORIGIN`, requires `JWT_SECRET`, sends OTP codes by email
when `SMTP_*` is set (and stops returning them in the response), and gates
`POST /dev/seed` behind `ADMIN_TOKEN`. List responses never include owner email
or contact details. Still recommended before heavy use: request throttling on
`request-otp`, and scheduled jobs for 30-day listing expiry and the moderation
queue.

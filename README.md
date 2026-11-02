# Campus Circle

**Live:** <https://cc.siv19.dev>

Campus Circle is a student-run, privacy-first exchange for verified Purdue students: housing, marketplace listings, and rides in one responsive app. It was started after the founder personally experienced a scam and wanted student interactions to begin with stronger verification and safer defaults.

> Unofficial project. Campus Circle is not affiliated with, endorsed by, or operated by Purdue University.

## Monorepo

- `apps/web` — Next.js 16, React 19, TypeScript
- `apps/api` — Rust, Axum, official MongoDB driver
- `docker-compose.yml` — MongoDB 8 and the Rust API

## Run locally

Requirements: Node.js 20+ and Docker Desktop.

```bash
npm install
docker compose up -d --build
curl -X POST http://localhost:8081/dev/seed
npm run dev
```

Open [http://localhost:3000](http://localhost:3000). The development OTP is returned by the API and displayed in the verification dialog; replace this behavior with a transactional email provider before production.

## Environment

Copy `.env.example` to `.env` for the API and `apps/web/.env.local` for the client as needed. The MongoDB connection is configured with `MONGODB_URI`; Atlas and local MongoDB URIs are both supported.

## Implemented MVP

- Mobile-first responsive home and listing board
- Purdue email OTP verification and signed sessions
- Housing, marketplace, and ride categories with search/filtering
- Authenticated listing creation
- Consent-based contact requests
- In-app report flow and MongoDB moderation records
- MongoDB indexes, expiry fields, seeded demo data, and health endpoint
- Safety guidance and founder motivation without fabricated claims

## Production notes

Before launch, replace the development OTP response with ZeptoMail delivery, provide a strong `JWT_SECRET`, restrict CORS, add request throttling, and connect scheduled expiry/moderation jobs. The current API never returns owner email addresses or contact details in list responses.

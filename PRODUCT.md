# Campus Circle

<!-- impeccable:product-schema 1 -->

## Platform

web

## Stack

Next.js, React, TypeScript, and Tailwind CSS for the responsive web client. Rust for the API, in a single monorepo. MongoDB is the persistence layer, connected through the official Rust driver and configurable with `MONGODB_URI`.

## Users

Verified Purdue University students who need a safer, campus-scoped way to find subleases, buy and sell goods, and coordinate rides.

## Product Purpose

Unify three fragmented student workflows—housing, marketplace, and rides—inside a privacy-first community product. Success means students can discover, post, communicate, and moderate listings without exposing personal contact information publicly.

## Positioning

Access is gated by possession of a Purdue email address, while contact details remain consent-based and hidden until explicitly approved. Messaging happens in-app first.

## Operating Context

Students use the product on phones between classes and on laptops while planning housing, purchases, or travel. Listings are time-sensitive and should be fast to scan, filter, save, post, report, and resolve.

## Capabilities and Constraints

- Housing, marketplace, and rides listings with category-specific fields and filters.
- Purdue-email OTP verification; local development may expose a safe development OTP.
- In-app messages and consent-based contact requests.
- Reporting and an admin moderation queue.
- No payments, live location tracking, public student directory, or publicly returned contact fields.
- Student-run and unofficial. Never imply Purdue University endorsement and never use Purdue logos or official branding.
- Listings expire after 30 days unless renewed. Sold or expired records remain auditable.
- Rust replaces the originally proposed Python API and worker stack.
- MongoDB must be used through an environment-configured connector; local development includes a containerized MongoDB option.

## Brand Commitments

The working name is Campus Circle. The voice is plainspoken, neighborly, reassuring, and candid about safety. A persistent disclaimer must state that the service is student-run and not affiliated with or endorsed by Purdue University.

## Evidence on Hand

The source product specification is `/Users/sivab/Downloads/README.md`. No testimonials, launch metrics, official partnerships, or university-provided assets exist; future work must not fabricate them.

The founder experienced a scam personally. The homepage may truthfully say the product was built in response to that experience, without inventing the circumstances, amount, or outcome.

## Product Principles

- Trust is demonstrated through visible product behavior, not vague claims.
- Private by default; disclosure requires explicit, reversible consent.
- Make the next useful action obvious on a small screen.
- Keep listings auditable without making personal data public.
- Distinguish student community identity from university affiliation.

## Accessibility & Inclusion

The app must remain keyboard-operable, readable with strong contrast, responsive at narrow mobile widths, and understandable without relying on color alone.

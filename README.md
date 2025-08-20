# Personal GitHub Dashboard

A personalized dashboard that provides project management, insights and analytics for GitHub repositories and activities.

## Features

- Repository overview with key metrics
- Activity analytics and visualizations
- Notification center
- Performance metrics dashboard
- Customizable layouts with dark/light theme

## Tech Stack

- Frontend: Vite, React, TypeScript, Tailwind CSS
- Backend: Rust, Actix-web (REST + WebSocket/SSE)
- Database: PostgreSQL
- Cache/Queue: Redis
- Observability: tracing + Prometheus (planned)

## Development Setup

### Prerequisites

- Node.js 18+
- Rust 1.75+
- PostgreSQL 14+ (local or Docker)
- Redis 6+ (local or Docker)
- Docker (optional, for local stack)

### Environment variables

- GITHUB_CLIENT_ID
- GITHUB_CLIENT_SECRET
- GITHUB_REDIRECT_URL (must match your GitHub OAuth app callback, e.g., http://localhost:3001/auth/callback)
- JWT_SECRET
- DATABASE_URL (e.g., postgres://postgres:postgres@localhost:5432/personal_github_dashboard)
- REDIS_URL (e.g., redis://localhost:6379)

### Getting Started

- Clone this repository
- Create a .env file at the repo root with the variables above
- Start PostgreSQL and Redis locally (or via Docker Compose when available)
- Run backend and frontend (will be added when the project is scaffolded)

## Project Structure

Planned structure (to be scaffolded):

```text
frontend/      # Vite + React + TS + Tailwind (web)
backend/       # Rust + Actix-web (API + background workers)
infra/         # DB migrations, seeds, scripts
docker/        # docker-compose for local dev
```

## Product Decisions

- Scope: Personal initially; consider team/org later based on demand
- Repos: Include private repos; org membership supported
- Notifications: In-app initially; email/Slack in later phase
- Hosting: Local-first; optional SaaS later
- Prioritization: Provide sensible defaults with onboarding to customize
 - OAuth scopes: Initial = read:user, user:email, repo, read:org; defer notifications until later

## API Endpoints

### Auth

- `GET /auth/login` - Redirect to GitHub OAuth
- `GET /auth/callback` - OAuth callback (uses GITHUB_REDIRECT_URL)
- `POST /auth/logout` - Clear session
- `GET /me` - Return current user profile/session

### Work

- `GET /api/work-items?type=pr|issue&state=open&sort=priority` - Unified "My Work" list

### Repositories

- `GET /api/repos` - List accessible/followed repositories
- `POST /api/repos/{id}/follow` - Follow/unfollow a repository

### Analytics

- `GET /api/analytics/repository/{owner}/{repo}/activity` - Repository activity data
- `GET /api/analytics/repository/{owner}/{repo}/trends` - Repository trends
- `GET /api/analytics/repository/{owner}/{repo}/summary` - Key metrics summary

### Data Synchronization

- `POST /api/sync/repository/{owner}/{repo}` - Manually trigger repository data sync

### Health Check

- `GET /api/health` - Check service health status

### Realtime

- `GET /api/stream` - Server-Sent Events for live updates (or `/ws` for WebSocket)

## Authorization & Scopes (GitHub OAuth)

Initial scopes (MVP):

- read:user
- user:email
- repo (private repos, issues/PRs)
- read:org (org membership/access checks)

Deferred scopes (later):

- notifications (enable GitHub notifications integration in a later phase)

## Prioritization Model (default)

- Assignment to you: +3
- Review requested: +4
- Severity labels (P0/P1/bug/security): +5/+3/+3/+5
- Staleness: +1/day after 2 days (cap +7)
- Near due (<48h): +4; overdue: +6
- Small PR (≤200 LOC): +2
- Blocked (needs author/tests failing): +3
- Draft PR: −3; WIP label: −2

### Data flow:

1. GitHub data is fetched via API
2. Stored in PostgreSQL database
3. Cached in Redis for performance
4. Processed for analytics
5. Served via REST API and WebSocket
6. Visualized in web interface


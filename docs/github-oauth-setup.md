# GitHub OAuth Application Setup

## Application Details

- **App Name:** Personal GitHub Dashboard
- **Homepage URL:** http://localhost:3000
- **Callback URL:** http://localhost:3000/api/auth/callback

## Required Scopes

- read:org
- read:user
- repo

## Environment Variables

Copy `.env.example` to `.env` and update the following variables with your credentials:

- `GITHUB_CLIENT_ID`
- `GITHUB_CLIENT_SECRET`
- `GITHUB_REDIRECT_URL` # <-- Only this variable is required
- `GITHUB_OAUTH_SCOPES`

**Note:** Do not store any credentials in documentation or anywhere outside the .env files. Never commit secrets to a public repository.

## Management

- Registered at: https://github.com/settings/developers
- Only project admins should access or rotate client secrets.

---

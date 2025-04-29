-- Migration: Create oauth_tokens table for storing encrypted OAuth tokens
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE oauth_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider VARCHAR(32) NOT NULL DEFAULT 'github',
    access_token BYTEA NOT NULL, -- encrypted
    refresh_token BYTEA,         -- encrypted
    token_type VARCHAR(32),
    scope VARCHAR(255),
    expiry TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC')
);

CREATE INDEX idx_oauth_tokens_user_id ON oauth_tokens(user_id);
CREATE INDEX idx_oauth_tokens_provider ON oauth_tokens(provider);

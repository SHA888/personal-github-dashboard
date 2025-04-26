-- Drop and recreate organizations, repositories, and activities tables with UUID PKs and UUID FKs
-- This migration is destructive: all data will be lost unless migrated.

-- ORGANIZATIONS TABLE
DROP TABLE IF EXISTS organizations CASCADE;
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC')
);

-- REPOSITORIES TABLE
DROP TABLE IF EXISTS repositories CASCADE;
CREATE TABLE repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID REFERENCES organizations(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    language VARCHAR(50),
    stars INTEGER,
    forks INTEGER,
    issues INTEGER,
    owner_id UUID REFERENCES users(id) ON DELETE SET NULL,
    is_private BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC')
);
CREATE INDEX idx_repositories_org_id ON repositories(org_id);
CREATE INDEX idx_repositories_owner_id ON repositories(owner_id);

-- ACTIVITIES TABLE
DROP TABLE IF EXISTS activities CASCADE;
CREATE TABLE activities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    repo_id UUID REFERENCES repositories(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    data JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC')
);
CREATE INDEX idx_activities_user_id ON activities(user_id);
CREATE INDEX idx_activities_repo_id ON activities(repo_id);

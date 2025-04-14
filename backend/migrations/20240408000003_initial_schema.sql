-- Add migration script here

-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    github_id BIGINT NOT NULL UNIQUE,
    login VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    email VARCHAR(255),
    avatar_url TEXT,
    html_url TEXT,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    last_synced_at TIMESTAMPTZ,
    UNIQUE(github_id)
);

-- Create organizations table
CREATE TABLE IF NOT EXISTS organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    github_id BIGINT NOT NULL UNIQUE,
    login VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    description TEXT,
    avatar_url TEXT,
    html_url TEXT,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    last_synced_at TIMESTAMPTZ,
    UNIQUE(github_id)
);

-- Create repositories table
CREATE TABLE IF NOT EXISTS repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    github_id BIGINT NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    description TEXT,
    private BOOLEAN,
    fork BOOLEAN,
    html_url TEXT,
    clone_url TEXT,
    default_branch VARCHAR(255),
    language VARCHAR(100),
    stargazers_count INTEGER,
    watchers_count INTEGER,
    forks_count INTEGER,
    open_issues_count INTEGER,
    size INTEGER,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    pushed_at TIMESTAMPTZ,
    last_synced_at TIMESTAMPTZ,
    UNIQUE(github_id)
);

-- Create organization_members table
CREATE TABLE IF NOT EXISTS organization_members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(organization_id, user_id)
);

-- Create repository_collaborators table
CREATE TABLE IF NOT EXISTS repository_collaborators (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(repository_id, user_id)
);

-- Add indexes for common queries
CREATE INDEX IF NOT EXISTS idx_organizations_name_login ON organizations(name, login);
CREATE INDEX IF NOT EXISTS idx_repositories_name ON repositories(name);
CREATE INDEX IF NOT EXISTS idx_users_login ON users(login);
CREATE INDEX IF NOT EXISTS idx_org_members_org_id ON organization_members(organization_id);
CREATE INDEX IF NOT EXISTS idx_repo_collabs_repo_id ON repository_collaborators(repository_id);

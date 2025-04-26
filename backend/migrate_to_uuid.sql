-- DESTRUCTIVE MIGRATION: Convert all PKs and FKs to UUID

-- Enable UUID generation
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Drop Foreign Key Constraints
ALTER TABLE activities DROP CONSTRAINT IF EXISTS activities_repo_id_fkey;
ALTER TABLE activities DROP CONSTRAINT IF EXISTS activities_user_id_fkey;
ALTER TABLE repositories DROP CONSTRAINT IF EXISTS repositories_org_id_fkey;
ALTER TABLE repositories DROP CONSTRAINT IF EXISTS repositories_owner_id_fkey;

-- Remove all data
TRUNCATE TABLE activities, repositories, organizations, users RESTART IDENTITY CASCADE;

-- USERS: id to UUID
ALTER TABLE users ALTER COLUMN id DROP DEFAULT;
ALTER TABLE users ALTER COLUMN id TYPE uuid USING (gen_random_uuid());
ALTER TABLE users ALTER COLUMN id SET DEFAULT gen_random_uuid();

-- ORGANIZATIONS: id to UUID
ALTER TABLE organizations ALTER COLUMN id DROP DEFAULT;
ALTER TABLE organizations ALTER COLUMN id TYPE uuid USING (gen_random_uuid());
ALTER TABLE organizations ALTER COLUMN id SET DEFAULT gen_random_uuid();

-- REPOSITORIES: org_id and owner_id to UUID
ALTER TABLE repositories ALTER COLUMN org_id TYPE uuid USING (NULL);
ALTER TABLE repositories ALTER COLUMN owner_id TYPE uuid USING (NULL);

-- ACTIVITIES: user_id to UUID
ALTER TABLE activities ALTER COLUMN user_id TYPE uuid USING (NULL);

-- Remove old sequences
DROP SEQUENCE IF EXISTS users_id_seq CASCADE;
DROP SEQUENCE IF EXISTS organizations_id_seq CASCADE;

-- Re-create Foreign Key Constraints
ALTER TABLE activities
    ADD CONSTRAINT activities_repo_id_fkey FOREIGN KEY (repo_id) REFERENCES public.repositories(id) ON DELETE CASCADE;
ALTER TABLE activities
    ADD CONSTRAINT activities_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;
ALTER TABLE repositories
    ADD CONSTRAINT repositories_org_id_fkey FOREIGN KEY (org_id) REFERENCES public.organizations(id) ON DELETE SET NULL;
ALTER TABLE repositories
    ADD CONSTRAINT repositories_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.users(id) ON DELETE SET NULL;

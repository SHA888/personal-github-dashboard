-- migrate:up

-- Drop all existing tables and extensions
DROP EXTENSION IF EXISTS "uuid-ossp" CASCADE;

-- Drop all existing tables in the correct order to handle dependencies
DROP TABLE IF EXISTS _sqlx_migrations CASCADE;
DROP TABLE IF EXISTS activity_types CASCADE;
DROP TABLE IF EXISTS repository_collaborators CASCADE;
DROP TABLE IF EXISTS organization_members CASCADE;
DROP TABLE IF EXISTS commits CASCADE;
DROP TABLE IF EXISTS analytics_data CASCADE;
DROP TABLE IF EXISTS tasks CASCADE;
DROP TABLE IF EXISTS task_assignees CASCADE;
DROP TABLE IF EXISTS task_labels CASCADE;
DROP TABLE IF EXISTS notifications CASCADE;
DROP TABLE IF EXISTS repositories CASCADE;
DROP TABLE IF EXISTS organizations CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS sync_status CASCADE;

-- migrate:down

-- No down migration needed for cleanup 
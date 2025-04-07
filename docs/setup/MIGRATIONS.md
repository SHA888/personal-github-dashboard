# Database Migrations

This document describes the database migrations used in the Personal GitHub Dashboard project.

## Overview

The project uses SQLx migrations to manage database schema changes. Migrations are stored in the `backend/migrations` directory and are automatically applied when the application starts.

## Migration Files

### 20240406000000_clean_schema.sql

Initial schema setup with core tables:

1. **repositories**
   - Primary table for storing GitHub repository information
   - Fields: id, owner, name, created_at, updated_at
   - Unique constraint on (owner, name)

2. **commits**
   - Stores commit information for repositories
   - Fields: id, repository_id, sha, message, author_name, author_email, created_at
   - Foreign key to repositories table
   - Unique constraint on (repository_id, sha)

3. **analytics_data**
   - Stores various analytics metrics
   - Fields: id, repository_id, metric_type, value, created_at
   - Foreign key to repositories table

4. **Indexes**
   - idx_repositories_owner_name: Optimizes repository lookups
   - idx_commits_repository_id: Optimizes commit queries by repository
   - idx_commits_created_at: Optimizes time-based commit queries
   - idx_analytics_data_repository_id: Optimizes analytics queries
   - idx_analytics_data_metric_type: Optimizes metric type lookups
   - idx_analytics_data_created_at: Optimizes time-based analytics queries

### 20240407000000_enhance_schema.sql

Schema enhancements and additional features:

1. **Repository Metadata**
   - Added fields to repositories table:
     - description: Repository description
     - language: Primary programming language
     - stars: Number of stars
     - forks: Number of forks
     - open_issues: Number of open issues
     - is_private: Repository visibility

2. **Activity Types**
   - New table for activity type definitions
   - Pre-populated with common activity types:
     - commit
     - pull_request
     - issue
     - review

3. **Task Management**
   - New tables for task tracking:
     - tasks: Main task table
     - task_assignees: Many-to-many relationship for task assignments
     - task_labels: Many-to-many relationship for task labels

4. **Notifications**
   - New table for real-time notifications
   - Fields: id, repository_id, type, message, created_at, is_read

5. **Additional Indexes**
   - Task-related indexes for performance optimization
   - Activity tracking indexes
   - Notification indexes

## Running Migrations

Migrations are automatically applied when the application starts. The process:

1. Checks for existing migrations in the database
2. Applies any pending migrations in order
3. Records migration status in the `_sqlx_migrations` table

To manually run migrations:

```bash
cd backend
cargo sqlx migrate run
```

To create a new migration:

```bash
cd backend
cargo sqlx migrate add <migration_name>
```

## Best Practices

1. **Versioning**
   - Use timestamp-based versioning (YYYYMMDDHHMMSS)
   - Ensure version numbers are unique and sequential

2. **Idempotency**
   - All migrations should be idempotent
   - Use `IF NOT EXISTS` and `IF EXISTS` clauses
   - Handle conflicts appropriately

3. **Data Safety**
   - Include appropriate foreign key constraints
   - Use appropriate data types and constraints
   - Add indexes for performance optimization

4. **Documentation**
   - Document schema changes in this file
   - Include comments in migration files
   - Document any data transformations

## Troubleshooting

If you encounter migration issues:

1. Check the `_sqlx_migrations` table for applied migrations
2. Verify migration file versions are unique
3. Ensure all migrations are idempotent
4. Check for syntax errors in SQL files
5. Verify database permissions

## Future Migrations

When adding new migrations:

1. Follow the existing naming convention
2. Include appropriate indexes
3. Document changes in this file
4. Test migrations in a development environment
5. Consider backward compatibility

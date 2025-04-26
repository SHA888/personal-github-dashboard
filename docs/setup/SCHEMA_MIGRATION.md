# Database Schema Migration (2025-04-22)

## Overview

This migration consolidates all previous schema changes into a single, unified migration for the development phase. It defines the following tables for the GitHub Dashboard backend:

- **users**: Core user data, GitHub linkage, tokens, preferences
- **organizations**: Organization info, GitHub linkage, member count
- **repositories**: Repository metadata, linkage to orgs and owners
- **activities**: User/repo activity log, extensible JSON payload

## Key Features

- All tables include `created_at` and `updated_at` (UTC)
- Proper foreign key constraints and indexes for relational integrity and query performance
- Uses appropriate data types for scalability and compatibility with analytics features

## Migration File

- Location: `backend/migrations/20250422060000_full_schema.sql`
- Old migration files removed for clarity during development

## How to Apply

1. Ensure your database is up and running (PostgreSQL 15+)
2. From the `backend/` directory, run:
   ```bash
   sqlx migrate run
   ```
3. This will apply the consolidated schema to your database.

## Notes

- This approach is for development. For production, consider using incremental migrations for audit/history.
- Update environment variables as needed in `.env` or Docker Compose for database connectivity.

---

For questions, see `backend/SCHEMA_MIGRATION.md` or ask in the project chat.

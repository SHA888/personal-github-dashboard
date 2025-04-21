#!/bin/bash
# Usage: ./scripts/dev_reset_db_and_migrate.sh
# Drops, recreates, and migrates the dev database using Docker Compose config

set -e

# Always run from project root
dirname=$(dirname "$0")
cd "$dirname/.."

# Load DB credentials from .env.db if present (do NOT source .env)
if [ -f .env.db ]; then
  set -a
  source .env.db
  set +a
fi

DB_NAME=${POSTGRES_DB:-personal_github_dashboard}
DB_USER=${POSTGRES_USER:-postgres}
DB_PASSWORD=${POSTGRES_PASSWORD:-postgres}
DB_HOST=${POSTGRES_HOST:-localhost}
DB_PORT=${POSTGRES_PORT:-5432}

# Check if docker-compose.yml exists in project root
if [ ! -f docker-compose.yml ]; then
  echo "docker-compose.yml not found in project root. Please run from the project root directory."
  exit 1
fi

# Drop and recreate the database
PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -p "$DB_PORT" -c "DROP DATABASE IF EXISTS $DB_NAME;"
PGPASSWORD="$DB_PASSWORD" psql -h "$DB_HOST" -U "$DB_USER" -p "$DB_PORT" -c "CREATE DATABASE $DB_NAME;"

# Run migrations
cd backend
sqlx migrate run

echo "Database $DB_NAME reset and migrations applied."

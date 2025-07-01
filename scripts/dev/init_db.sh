#!/usr/bin/env bash
set -eo pipefail

DB_USER="${DB_USER:-admin}"
DB_PASSWORD="${DB_PASSWORD:-postgres}"
DB_NAME="${DB_NAME:-medpass-dev}"
DB_PORT="${DB_PORT:-5432}"
DB_HOST="${DB_HOST:-localhost}"
DB_CONTAINER_NAME="dev_postgres"
DATA_DIR="${DATA_DIR:-${PWD}/pgdata}"

echo "⏹️  Stopping and removing any previous '${DB_CONTAINER_NAME}' container (if exists)..."
docker rm -f "${DB_CONTAINER_NAME}" > /dev/null 2>&1 || true

echo "🚀 Starting Postgres container '${DB_CONTAINER_NAME}'..."
docker run \
  --name "${DB_CONTAINER_NAME}" \
  -e POSTGRES_USER="${DB_USER}" \
  -e POSTGRES_PASSWORD="${DB_PASSWORD}" \
  -e POSTGRES_DB="${DB_NAME}" \
  -p "${DB_PORT}":5432 \
  -v "${DATA_DIR}":/var/lib/postgresql/data \
  -d postgres \
  postgres -N 1000

echo "⏳ Waiting for Postgres to be ready on port ${DB_PORT}..."
until docker exec "${DB_CONTAINER_NAME}" pg_isready -U "${DB_USER}" > /dev/null 2>&1; do
  sleep 1
done

echo "✅ Postgres is ready and running in Docker."

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL

echo "🎯 Running sqlx database create..."
if ! sqlx database create; then
  echo "❌ sqlx failed. Stopping and removing Docker container '${DB_CONTAINER_NAME}'..."
  docker rm -f "${DB_CONTAINER_NAME}"
  exit 1
fi
sqlx migrate run

echo "✅ Database created successfully."
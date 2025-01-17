#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if sqlx is installed
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@localhost:${POSTGRES_PORT}/${POSTGRES_DB}
sqlx migrate add create_users_table
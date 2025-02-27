#!/usr/bin/env bash
set -e

# Core
cargo test -p sqlx-core --all-features

# Postgres
env DATABASE_URL="postgres://" cargo test -p sqlx --no-default-features --features 'postgres uuid chrono'

# MySQL
env DATABASE_URL="mysql:///sqlx" cargo test -p sqlx --no-default-features --features 'mysql'

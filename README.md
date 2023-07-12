# cm_zero2prod

brew install michaeleisel/zld/zld

-- Rust

cargo install cargo-watch
cargo install cargo-tarpaulin

cargo watch -x check -x test -x run

-- Postgres init script

chmod +x scripts/init_db.sh
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

-- psql install 

brew install postgresql
brew services list
brew services start postgresql

-- sqlx migrations 

sqlx migrate add create_subscriptions_table

-- Docker

SKIP_DOCKER=true ./scripts/init_db.sh
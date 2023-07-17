# cm_zero2prod

brew install michaeleisel/zld/zld

-- Rust

cargo install cargo-watch
cargo install cargo-tarpaulin

cargo watch -x check -x test -x run

RUST_LOG=debug cargo run
RUST_LOG=trace cargo run

-- Docker 

# Build a docker image tagged as "zero2prod" according to the recipe
# specified in `Dockerfile`
docker build --tag zero2prod --file Dockerfile .
docker builder prune
docker run -p 8000:8000 --net=bridge --detach zero2prod 

-- Tests

cargo install bunyan
TEST_LOG=true cargo test health_check_works | bunyan
TEST_LOG=true cargo test health_check_works
cargo test

-- Postgres init script

chmod +x scripts/init_db.sh
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
cargo sqlx prepare -- --lib

-- psql install 

brew install postgresql
brew services list
brew services start postgresql

-- sqlx migrations 

sqlx migrate add create_subscriptions_table

-- Docker

SKIP_DOCKER=true ./scripts/init_db.sh

-- Requests

curl -d "name=john11&email=john11.doe@gmail.com" -H "Content-Type: application/x-www-form-urlencoded" -X POST http://localhost:8000/subscriptions --verbose

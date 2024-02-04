clear
docker compose up -d --remove-orphans --build
cargo fmt
source ./postgres_secrets.env; cargo run
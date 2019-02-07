docker-compose up -d
cd integration
cargo test
docker-compose down
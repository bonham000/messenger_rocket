#!/usr/bin/env bash

docker-compose up -d
cd integration && cargo test -- --nocapture
#docker-compose down
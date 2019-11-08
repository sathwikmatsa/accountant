#!/bin/sh
cargo install diesel_cli --no-default-features --features sqlite

read -p "enter database name: " dbname
echo "DATABASE_URL=$dbname" > .env

diesel migration run

cargo build

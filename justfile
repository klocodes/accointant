run:
    PROJECT_ROOT=$(pwd) RUST_BACKTRACE="full" RUST_LOG=actix_web=debug cargo run

migrate-create *options:
    just migrate create -dir db/migrations -ext sql {{options}}

migrate *options:
    @source ./db/set_db_url.sh; migrate -path ./db/migrations -database "$DB_URL" {{options}}

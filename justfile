run:
    PROJECT_ROOT=$(pwd) RUST_BACKTRACE="full" RUST_LOG=actix_web=debug cargo run

test:
    PROJECT_ROOT=$(pwd) RUST_LOG=actix_web=debug cargo test

release:
    PROJECT_ROOT=$(pwd) RUST_BACKTRACE="full" RUST_LOG=actix_web=debug cargo run --release

create-migration *options:
    just migrate create -dir db/migrations -ext sql {{options}}

migrate *options:
    @source ./db/set_db_url.sh; migrate -path ./db/migrations -database "$DB_URL" {{options}}

test-migrate db_url *options:
    migrate -path ./db/migrations -database {{db_url}} {{options}}

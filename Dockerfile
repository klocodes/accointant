# Этап 1: Сборка приложения
FROM rust:1.64-alpine as builder
WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

# Этап 2: Создание окончательного образа
FROM alpine:latest

RUN apk add --no-cache libgcc

COPY --from=builder /usr/src/myapp/target/release/myapp /usr/local/bin/myapp

CMD ["myapp"]

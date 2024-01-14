# Этап 1: Сборка приложения
ARG RUST_VERSION=1.73

FROM rust:${RUST_VERSION} as builder

ARG TARGET=release

COPY . /build

WORKDIR /build

RUN cargo install --path .

# Собираем релизную версию приложения
RUN if [ "$TARGET" = "release" ]; then cargo build --release; else cargo build; fi


# Этап 2: Создание окончательного образа
FROM debian:bookworm-slim

ARG TARGET=release

RUN mkdir /app

# Копируем собранное приложение из каталога target/release
COPY --from=builder /build/target/${TARGET}/metan /app

RUN rm -rf /build

COPY config /app/config

COPY .env /app/.env

RUN mkdir /app/log

# Указываем команду для запуска приложения
CMD ["/app/metan"]

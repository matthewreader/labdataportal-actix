FROM rust:1.71.0 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
ENV APP_ENVIRONMENT production
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/labdataportal-actix labdataportal-actix

COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./labdataportal-actix"]
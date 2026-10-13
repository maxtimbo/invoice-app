# ---- build ----
FROM rust:1-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release -p invoice-web \
    && cp target/release/invoice-web /usr/local/bin/invoice-web

# ---- runtime ----
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates libssl3 chromium fonts-liberation fonts-dejavu-core \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --uid 10001 invoice \
    && mkdir -p /data && chown invoice:invoice /data

COPY --from=build /usr/local/bin/invoice-web /usr/local/bin/invoice-web
COPY crates/invoice-web/templates /app/templates
COPY crates/invoice-web/static /app/static

ENV INVOICE_DATA_DIR=/data \
    INVOICE_WEB_DIR=/app \
    INVOICE_BIND=0.0.0.0:3000 \
    INVOICE_CHROME_NO_SANDBOX=1 \
    CHROME=/usr/bin/chromium

USER invoice
VOLUME /data
EXPOSE 3000
CMD ["invoice-web"]

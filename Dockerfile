FROM rust:1.78 AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin zero_trust_auth

# our final base
FROM debian:bookworm
WORKDIR /app

COPY --from=builder /app/target/release/zero_trust_auth zero_trust_auth
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero_trust_auth"]
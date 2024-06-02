FROM rust:1.78 as build
WORKDIR /build
COPY . .
RUN cargo build --release

# our final base
FROM debian:bookworm
WORKDIR /app

# copy the build artifact from the build stage
COPY --from=build /build/target/release/zero_trust_auth .
COPY --from=build /build/configuration.yaml .

# set the startup command to run your binary
CMD ["./zero_trust_auth"]
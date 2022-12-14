FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM gcr.io/distroless/cc
LABEL version="0.2.0" description="The exporter for SwitchBot made in Rust" maintainer="JichouP"
WORKDIR /app
COPY --from=builder /app/target/release/switchbot-exporter /usr/local/bin/switchbot-exporter
COPY ./Rocket.toml /app/Rocket.toml
ENTRYPOINT ["/usr/local/bin/switchbot-exporter"]
EXPOSE 8000

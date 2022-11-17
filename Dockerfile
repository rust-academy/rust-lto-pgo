# https://hub.docker.com/_/rust
ARG BASE_IMAGE=rust:1.65.0-slim-bullseye
FROM $BASE_IMAGE as base
RUN apt-get update
RUN apt-get install -y pkg-config libudev-dev librust-openssl-sys-dev

FROM base as builder
# Build caching
# https://stackoverflow.com/questions/42130132/can-cargo-download-and-build-dependencies-without-also-building-the-application

# [builder 1/8] Create dummy crate
RUN cargo new app

# [builder 2/8] Set WORKDIR
WORKDIR app

# Add project dependencies. These will be cached as long as they remained unchanged.
# [builder 3/8] ADD Cargo.toml
ADD Cargo.toml /app/Cargo.toml

# [builder 4/8] ADD Cargo.lock
ADD Cargo.lock /app/Cargo.lock

# [builder 5/8] Build project dependencies only
RUN cargo build --release

# [builder 6/8] Remove dummy sources
RUN rm -rf src/

# [builder 7/8] Add project sources
ADD src/ src/

# [builder 8/8]  Build project sources with dependencies already compiled
RUN cargo build --release

FROM gcr.io/distroless/cc
# Copy over the binary
COPY --from=builder /app/target/release/app /
EXPOSE 8080
CMD ["./app"]
# Rust as the base image
FROM rust:1.65 as build

# Create a new empty shell project
RUN USER=root cargo new --bin rusty_snake
WORKDIR /rusty_snake

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/chicken_in_a_biskitsnake*
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /rusty_snake/target/release/chicken-in-a-biskitsnake /usr/src/chicken-in-a-biskitsnake
# COPY --from=build /rusty_snake/target/release/rusty_snake/target/x86_64-unknown-linux-musl/release/rusty_snake .

# Run the binary
CMD ROCKET_ADDR=0.0.0.0 /usr/src/chicken-in-a-biskitsnake
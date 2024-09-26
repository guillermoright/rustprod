# We use the latest Rust stable release as base image
FROM rust:1.81-slim-bullseye
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev
WORKDIR /app
# Install the required system dependencies for our linking configuration
COPY . .

RUN cargo build --locked --release
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./target/release/zero2prod"]
# This Dockerfile is meant to be used to deploy on https://fly.io
# Source: https://github.com/fly-apps/hello-rust
# If you prefer a more generic one, use `Dockerfile.default`

###################### BACKEND BUILD ###################################################
FROM rust:1.63-bullseye AS backend_build

# Make a fake Rust app to keep a cached layer of compiled crates
RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# Needs at least a main.rs file with a main function
RUN mkdir src && echo "fn main(){}" > src/main.rs

# Build all dependent crates in release mode
RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/src/app/target \
  cargo build --release

# Copy the rest
COPY . .
# Build (install) the actual binaries
RUN cargo install --path .

###################### FRONTEND BUILD ##################################################
FROM node:16-alpine AS frontend_build

COPY ./frontend /app/frontend

WORKDIR /app/frontend
RUN npm install
RUN npm run build && npm run deploy

###################### RUNTIME IMAGE ###################################################
FROM debian:bullseye-slim

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends ca-certificates openssl \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*WORKDIR

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=backend_build /usr/local/cargo/bin/robo_radio /app/robo_radio
COPY --from=frontend_build /app/assets /app/assets

# No CMD or ENTRYPOINT, see fly.toml with `cmd` override
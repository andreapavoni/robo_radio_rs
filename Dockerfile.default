FROM rust:1.63-bullseye AS backend_build

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM node:16-alpine AS frontend_build

COPY ./frontend /app/frontend

WORKDIR /app/frontend
RUN npm install
RUN npm run build && npm run deploy

FROM debian:bullseye-slim

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends ca-certificates openssl \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*WORKDIR

COPY --from=backend_build /app/target/release/robo_radio /
COPY --from=frontend_build /app/assets /assets

CMD ./robo_radio
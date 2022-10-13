###################### FRONTEND BUILD ##########################################
FROM node:16-alpine AS frontend_build

COPY ./frontend /app/frontend

WORKDIR /app/frontend
RUN npm install
RUN npm run build && npm run deploy

###################### BACKEND BUILD ###########################################
FROM rust:1.63-bullseye AS backend_build

ENV CARGO_NET_GIT_FETCH_WITH_CLI=true

WORKDIR /app
COPY . /app
RUN cargo build --release

###################### RUNTIME IMAGE ###########################################
FROM debian:bullseye-slim

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends ca-certificates openssl \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*WORKDIR

COPY --from=backend_build /app/target/release/robo_radio /
COPY --from=frontend_build /app/assets /assets

CMD ./robo_radio

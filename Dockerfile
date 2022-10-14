###################### FRONTEND BUILD ##########################################
FROM node:16-alpine AS frontend_build

COPY ./frontend /app/frontend

WORKDIR /app/frontend
RUN npm install
RUN npm run build && npm run deploy

###################### BACKEND BUILD ###########################################
FROM rust:1.63-bullseye AS backend_build

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

# Run as "app" user
RUN useradd -ms /bin/bash app

USER app
WORKDIR /app

COPY --from=backend_build /app/target/release/robo_radio /app/robo_radio
COPY --from=frontend_build /app/assets /app/assets

CMD ["./robo_radio"]

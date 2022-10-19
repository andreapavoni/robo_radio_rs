# RoboRadio

## What is this

A Rust implementation of https://radio.pavonz.com (originally built with Elixir and Phoenix, still not open sourced yet). It works like a real radio: you turn it on (well, in this case, you open the web page) and you'll listen what's being played at the exact point. The radio goes on automagically.

## How it works

RoboRadio loads a playlist from https://soundcloud.com, shuffles the tracks, for each one it will play the song and notify the connected users through websockets. When the songs in the playlist have been all played, it reloads and shuffle the tracks again.

## How to run RoboRadio

### Locally

- clone the repository and run:
  ```sh
  $ cd frontend && npm install && cd ..
  $ cargo make frontend
  $ cargo run
  ```
- or use the `Dockerfile.default` to build a container and run it throught Docker (or Podman). The app will listen on port `8080`. There's a `.env.dist` file with usable ENV settings.

### Deploy

Use the `Dockerfile` included in this repository for a basic deploy, or adjust if needed. After some weeks of testing, I decided to deploy as the official RoboRadio in place of the Elixir one.

## Current status

- [x] add soundcloud API client
  - [x] fetch playlist info
  - [x] fetch track info
  - [x] fetch track stream
- [x] media player
  - [x] load playlist and randomize its tracks
  - [x] rotate to next track (or reload playlist again)
- [x] http server
  - [x] wrap app-state
  - [x] serve static assets
  - [x] websockets
    - [x] keep track of current listeners
    - [x] broadcast new track when last one's duration has elapsed
- [x] logging/tracing
- [x] improved error handling (`anyhow` + `thiserror` ?)
- [x] auto-update soundcloud's `client_id`
- [ ] testing (mocks for external API calls)

# RoboRadio

A Rust implementation of https://radio.pavonz.com (build with Elixir and Phoenix).

## Current status: WIP

Planned TODO:

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
- [ ] improved error handling (`anyhow` + `thiserror` ?)
- [ ] testing (mocks for external API calls)

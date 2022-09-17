# robo_radio_rs

A Rust implementation for https://radio.pavonz.com

## Current status: WIP

Planned TODO:

- [x] add soundcloud API client
  - [x] fetch playlist info
  - [x] fetch track info
  - [x] fetch track stream
- [x] media player
  - [x] load playlist and randomize its tracks
  - [x] rotate to next track (or reload playlist again)
- [ ] http server
  - [ ] wrap app-state
  - [ ] serve static assets
  - [ ] websockets
    - [ ] keep track of current listeners
    - [ ] broadcast new track when last one's duration has elapsed
- [ ] logging
- [ ] improved error handling (`anyhow` + `thiserror` ?)
- [ ] testing (mocks for external API calls)

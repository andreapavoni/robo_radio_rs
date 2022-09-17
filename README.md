# robo_radio_rs

A Rust implementation for https://radio.pavonz.com

## Current status: WIP

Planned TODO:

- [x] add soundcloud API client
  - [x] fetch playlist info
  - [x] fetch track info
  - [x] fetch track stream
- [ ] media player
  - [ ] start a new song after the duration of the previous one is elapsed
- [ ] http server
  - [ ] wrap app-state
  - [ ] serve static assets
  - [ ] websockets
- [ ] improved error handling (`anyhow` + `thiserror` ?)
- [ ] testing (mocks for external API calls)

import Player from "./lib/player";
import WS from "./lib/ws";

let player = new Player();
let protocol = location.protocol.match(/^https/) ? "wss" : "ws";
const url = `${protocol}://${location.host}/ws`;
let socket = new WS(url, "", 20000, 10000, 3000, "PING");

socket.onopen = function () {
  console.log(`connected to ws ${url}`);
};

socket.onmessage = function (e) {
  if (e.data == "PONG") {
    return;
  }

  try {
    let evt = JSON.parse(e.data);

    if (evt.event == "track") {
      player.load(evt.data);
      setMediaSession(evt.data);
      navigator.mediaSession.playbackState = "playing";

      if (player.status == "playing") {
        player.play();
      }
    }

    if (evt.event == "listeners") {
      let listeners = document.querySelector("#listeners");
      listeners.innerHTML = evt.data;
    }
  } catch (err) {
    console.log(`error: unrecognized message from server: `, err);
  }
};

socket.onclose = function () {
  console.log("connection closed");
};

socket.onerror = function () {
  // console.log("some error happened");
};

socket.onreconnect = function () {
  console.log("reconnecting...");
};

window.addEventListener("resize", player.resize);
player.resize();

function setMediaSession(track) {
  if ("mediaSession" in navigator) {
    navigator.mediaSession.metadata = new MediaMetadata({
      title: track.title,
      artist: track.artist,
      album: "",
      // artwork: [
      //   {
      //     src: "https://example.com/assets/cover.jpeg",
      //     sizes: "256x256",
      //     type: "image/jpeg",
      //   },
      //   {
      //     src: "https://example.com/assets/cover.jpeg",
      //     sizes: "512x512",
      //     type: "image/jpeg",
      //   },
      // ],
    });
  }
}

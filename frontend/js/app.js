import Player from "./lib/player";
import WS from "./lib/ws";

let player = new Player();

let protocol = location.protocol.match(/^https/) ? "wss" : "ws";
let socket = new WS(
  `${protocol}://${location.host}/ws`,
  "",
  15000,
  10000,
  2000,
  "PING"
);

socket.onopen = function () {
  console.log("connect success");
  socket.send("hello server");
};

socket.onmessage = function (e) {
  if (e.data == "PONG") {
    console.log("PONG");
    return;
  }

  try {
    let evt = JSON.parse(e.data);

    if (evt.event == "new_track") {
      player.load(evt.data);

      if (player.status == "playing") {
        player.play();
      }
    }

    if (evt.event == "listeners") {
      // updated listeners count...
    }
  } catch (_err) {
    console.log(`error: unrecognized message from server: `, e);
  }
};

socket.onclose = function () {
  console.log("connection closed");
};

socket.onerror = function (e) {
  console.log("some error happened", e);
};

socket.onreconnect = function () {
  console.log("reconnecting...");
};

window.addEventListener("resize", player.resize);
player.resize();

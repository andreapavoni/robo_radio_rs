// Bring in Phoenix channels client library:
/*
import { Socket } from "phoenix";
import Player from "./player";

// let token = document
//   .querySelector("meta[name='csrf-token']")
//   .getAttribute("content");
// let socket = new Socket("/socket", { params: { token: token } });
let socket = new Socket("/socket", { params: {} });

// Connect to the socket:
socket.connect();
// Now that you are connected, you can join channels with a topic.
let channel = socket.channel("radio:listen", {});
let player = new Player();

socket.onmessage = (event) => {
	console.log("========== EVENT ==================", event)
}


channel.on("radio:new_song", (new_song) => {
	console.log("===========================")
  player.load(new_song);
  if (player.status == "playing") {
    player.play();
  }
});

channel.on("radio:welcome", (current_song) => {
  player.load(current_song);
});

channel.on("radio:listeners", (payload) => {
  let listeners = document.getElementById("listeners");
  listeners.innerHTML = payload.listeners;
});

channel
  .join()
  .receive("ok", (resp) => { console.log("======== connected ==============")})
  .receive("error", (resp) => {
    console.log("error connecting to radio", resp);
  });

window.addEventListener("resize", player.resize);
player.resize();

export default socket;
*/

import Player from "./player";

let player = new Player();

let socket = new WebSocket("ws://127.0.0.1:3000/socket/websocket");

socket.onopen = function () {
  socket.send("hello from the client");
};

socket.onmessage = function (message) {
  console.log("player_status", player.status);

  let data = JSON.parse(message.data);
  console.log("============= received: ", data);
  player.load(data.payload);

  if (player.status == "playing") {
    player.play();
  }
};

socket.onerror = function (error) {
  console.log("WebSocket error: ", error);
};

window.addEventListener("resize", player.resize);
player.resize();
export default socket;

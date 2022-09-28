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

let protocol = location.protocol.match(/^https/) ? "wss" : "ws";
let socket = new WebSocket(`${protocol}://${location.host}/ws`);

console.log(`${protocol}://${location.host}/ws`);

socket.onopen = function () {};

socket.onmessage = function (message) {
  let data = JSON.parse(message.data);
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

// Bring in Phoenix channels client library:
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

channel.on("radio:new_song", (new_song) => {
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
  .receive("ok", (resp) => {})
  .receive("error", (resp) => {
    console.log("error connecting to radio", resp);
  });

window.addEventListener("resize", player.resize);
player.resize();

export default socket;

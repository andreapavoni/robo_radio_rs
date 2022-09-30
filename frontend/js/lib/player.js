import { Howl, Howler } from "howler";
import SiriWave from "../../vendor/siriwave";

// Cache references to DOM elements.
[
  "track",
  "artist",
  "trackPermalink",
  "timer",
  "duration",
  "playBtn",
  "pauseBtn",
  "prevBtn",
  "nextBtn",
  "infoboxBtn",
  "volumeBtn",
  "progress",
  "bar",
  "wave",
  "loading",
  "infobox",
  "list",
  "volume",
  "barEmpty",
  "barFull",
  "sliderBtn",
].forEach(function (elm) {
  window[elm] = document.getElementById(elm);
});

// Setup the "waveform" animation.
let wave = new SiriWave({
  container: window.waveform,
  width: window.innerWidth,
  height: window.innerHeight * 0.3,
  cover: true,
  speed: 0.02,
  amplitude: 0.7,
  frequency: 2,
  color: "#CC3A00",
});

// Includes all methods for playing, skipping, updating the display, etc.
export default class Player {
  constructor() {
    this.status = "paused";
    this.currentSong;
    this.currentData;
    this.volume = localStorage.getItem("volume") || 1;
    let self = this;

    // Bind our player controls.
    window.playBtn.addEventListener("click", () => {
      self.play();
    });
    window.pauseBtn.addEventListener("click", () => {
      self.pause();
    });
    window.volumeBtn.addEventListener("click", () => {
      self.toggleVolume();
    });
    window.volume.addEventListener("click", () => {
      self.toggleVolume();
    });

    // Setup the event listeners to enable dragging of volume slider.
    window.barEmpty.addEventListener("click", (event) => {
      let per = event.layerX / parseFloat(window.barEmpty.scrollWidth);
      self.volume = per;
      localStorage.setItem("volume", per);
    });
    window.sliderBtn.addEventListener("mousedown", () => {
      window.sliderDown = true;
    });
    window.sliderBtn.addEventListener("touchstart", () => {
      window.sliderDown = true;
    });
    window.volume.addEventListener("mouseup", () => {
      window.sliderDown = false;
    });
    window.volume.addEventListener("touchend", () => {
      window.sliderDown = false;
    });

    const move = function (event) {
      if (window.sliderDown) {
        let x = event.clientX || event.touches[0].clientX;
        let startX = window.innerWidth * 0.05;
        let layerX = x - startX;
        let per = Math.min(
          1,
          Math.max(0, layerX / parseFloat(window.barEmpty.scrollWidth))
        );
        self.volume = per;
        localStorage.setItem("volume", per);
      }
    };
    window.volume.addEventListener("mousemove", move);
    window.volume.addEventListener("touchmove", move);

    // bind spacebar to play/pause
    document.addEventListener("keyup", (event) => {
      if (event.code === "Space") {
        if (!!self.currentSong && self.status == "playing") {
          self.pause();
        } else {
          self.play();
        }
      }
    });

    wave.start();
  }

  // Load song and initialize a new Howler instance
  load(song) {
    let self = this;

    if (self.currentSong && self.currentSong.playing()) {
      self.currentSong.stop();
    }

    self.currentData = song;
    self.currentSong = new Howl({
      volume: 0.6,
      src: [song.url],
      html5: true, // Force to HTML5 so that the audio can stream in (best for large files).
      preload: false,
      onplay: function () {
        // Display the duration.
        window.duration.innerHTML =
          "-" + self.formatTime(Math.round(self.currentSong.duration()));

        // Start updating the progress of the track.
        requestAnimationFrame(self.step.bind(self));

        // Start the wave animation if we have already loaded
        wave.container.style.display = "block";
        window.bar.style.display = "none";
        window.pauseBtn.style.display = "block";
      },
      onload: function () {
        // Start the wave animation.
        wave.container.style.display = "none";
        window.bar.style.display = "block";
        window.loading.style.display = "none";
      },
      onend: function () {
        // Stop the wave animation.
        wave.container.style.display = "none";
        window.bar.style.display = "block";
      },
      onpause: function () {
        // Stop the wave animation.
        wave.container.style.display = "none";
        window.bar.style.display = "block";
      },
      onstop: function () {
        // Stop the wave animation.
        wave.container.style.display = "none";
        window.bar.style.display = "block";
      },
      onseek: function () {
        // Start updating the progress of the track.
        requestAnimationFrame(self.step.bind(self));
      },
    });

    // Update the track display.
    window.track.innerHTML = song.title;
    window.track.setAttribute("href", self.currentData.permalink_url);

    window.artist.innerHTML = self.currentData.artist;
    window.artist.setAttribute("href", self.currentData.artist_permalink);
  }

  // Play a song.
  play() {
    let self = this;

    // Show the pause button.
    if (self.currentSong.state() === "loaded") {
      window.playBtn.style.display = "none";
      window.pauseBtn.style.display = "block";
    } else {
      window.loading.style.display = "block";
      window.playBtn.style.display = "none";
      window.pauseBtn.style.display = "none";
    }

    // calculate how many seconds are passed since the start of the song on server
    const date = self.currentData.started_at;
    const now = new Date().toISOString();
    const timeDiff = Math.abs((Date.parse(now) - Date.parse(date)) / 1000);

    // move song to correct time position
    self.currentSong.seek(timeDiff);

    // Begin playing the sound.
    self.currentSong.play();
    self.status = "playing";
  }

  // Pause the currently playing track.
  pause() {
    let self = this;
    // Puase the sound.
    self.currentSong.pause();
    self.status = "paused";

    // Show the play button.
    window.playBtn.style.display = "block";
    window.pauseBtn.style.display = "none";
  }

  // Set the volume and update the volume slider display.
  set volume(val) {
    let self = this;

    // Update the global volume (affecting all Howls).
    Howler.volume(val);

    // Update the display on the slider.
    let barWidth = (val * 90) / 100;
    window.barFull.style.width = barWidth * 100 + "%";
    window.sliderBtn.style.left =
      window.innerWidth * barWidth + window.innerWidth * 0.05 - 25 + "px";
  }

  // Seek to a new position in the currently playing track
  seek(per) {
    let self = this;
    // Get the Howl we want to manipulate.
    let sound = self.currentSong;

    // Convert the percent into a seek position.
    if (sound.playing()) {
      sound.seek(sound.duration() * per);
    }
  }

  // The step called within requestAnimationFrame to update the playback position.
  step() {
    let self = this;
    // Get the Howl we want to manipulate.
    let sound = self.currentSong;

    // Determine our current seek position.
    let seek = sound.seek() || 0;
    window.timer.innerHTML = self.formatTime(Math.round(seek));
    window.progress.style.width = ((seek / sound.duration()) * 100 || 0) + "%";

    // Show remaining duration
    window.duration.innerHTML =
      "-" + self.formatTime(Math.round(sound.duration() - seek));

    // If the sound is still playing, continue stepping.
    if (sound.playing()) {
      requestAnimationFrame(self.step.bind(self));
    }
  }

  // Toggle the infobox display on/off.
  toggleInfobox() {
    let self = this;
    let display = window.infobox.style.display === "block" ? "none" : "block";

    setTimeout(
      function () {
        window.infobox.style.display = display;
      },
      display === "block" ? 0 : 500
    );
    window.infobox.className = display === "block" ? "fadein" : "fadeout";
  }

  // Toggle the volume display on/off.
  toggleVolume() {
    let self = this;
    let display = window.volume.style.display === "block" ? "none" : "block";

    setTimeout(
      function () {
        window.volume.style.display = display;
      },
      display === "block" ? 0 : 500
    );
    window.volume.className = display === "block" ? "fadein" : "fadeout";
  }

  // Format the time from seconds to M:SS.
  formatTime(secs) {
    let minutes = Math.floor(secs / 60) || 0;
    let seconds = secs - minutes * 60 || 0;

    return minutes + ":" + (seconds < 10 ? "0" : "") + seconds;
  }

  // Update the height of the wave animation.
  // These are basically some hacks to get SiriWave.js to do what we want.
  resize() {
    let height = window.innerHeight * 0.3;
    let width = window.innerWidth;
    wave.height = height;
    wave.height_2 = height / 2;
    wave.MAX = wave.height_2 - 4;
    wave.width = width;
    wave.width_2 = width / 2;
    wave.width_4 = width / 4;
    wave.canvas.height = height;
    wave.canvas.width = width;
    wave.container.style.margin = -(height / 2) + "px auto";

    // Update the position of the slider.
    let sound = this.currentSong;
    if (!!sound) {
      let vol = sound.volume();
      let barWidth = vol * 0.9;
      window.sliderBtn.style.left =
        window.innerWidth * barWidth + window.innerWidth * 0.05 - 25 + "px";
    }
  }
}

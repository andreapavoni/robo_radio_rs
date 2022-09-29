// Simple WebSocket wrapper with heartbeat and reconnection
// adapted from https://github.com/zimv/websocket-heartbeat-js

export default class WS {
  constructor(
    url,
    protocols = "",
    pingTimeout = 15000,
    pongTimeout = 10000,
    reconnectTimeout = 2000,
    pingMsg = "heartbeat",
    repeatLimit = 50
  ) {
    this.opts = {
      url: url,
      protocols,
      pingTimeout: pingTimeout,
      pongTimeout: pongTimeout,
      reconnectTimeout: reconnectTimeout,
      pingMsg: pingMsg,
      repeatLimit: repeatLimit,
    };
    this.ws = null;
    this.repeat = 0;

    // override hook functions
    this.onclose = () => {};
    this.onerror = () => {};
    this.onopen = () => {};
    this.onmessage = () => {};
    this.onreconnect = () => {};

    this.createWebSocket();
  }

  createWebSocket() {
    try {
      if (this.opts.protocols) {
        this.ws = new WebSocket(this.opts.url, this.opts.protocols);
      } else {
        this.ws = new WebSocket(this.opts.url);
      }
      this.initEventHandle();
    } catch (e) {
      this.reconnect();
      throw e;
    }
  }

  initEventHandle() {
    let self = this;

    self.ws.onclose = () => {
      self.onclose();
      self.reconnect();
    };

    self.ws.onerror = () => {
      self.onerror();
      self.reconnect();
    };

    self.ws.onopen = () => {
      self.repeat = 0;
      self.onopen();
      self.heartCheck();
    };

    self.ws.onmessage = (event) => {
      self.onmessage(event);
      self.heartCheck();
    };
  }

  reconnect() {
    let self = this;

    if (self.opts.repeatLimit > 0 && self.opts.repeatLimit <= self.repeat)
      return;

    if (self.lockReconnect || self.forbidReconnect) return;

    self.lockReconnect = true;
    self.repeat++;
    self.onreconnect();

    setTimeout(() => {
      self.createWebSocket();
      self.lockReconnect = false;
    }, self.opts.reconnectTimeout);
  }

  send(msg) {
    this.ws.send(msg);
  }

  heartCheck() {
    this.heartReset();
    this.heartStart();
  }

  heartStart() {
    let self = this;

    if (this.forbidReconnect) return;
    this.pingTimeoutId = setTimeout(() => {
      this.ws.send(this.opts.pingMsg);
      this.pongTimeoutId = setTimeout(() => {
        this.ws.close();
      }, this.opts.pongTimeout);
    }, this.opts.pingTimeout);
  }

  heartReset() {
    clearTimeout(this.pingTimeoutId);
    clearTimeout(this.pongTimeoutId);
  }

  close() {
    this.forbidReconnect = true;
    this.heartReset();
    this.ws.close();
  }
}

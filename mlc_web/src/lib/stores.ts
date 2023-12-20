import { readable } from "svelte/store";

export const info = readable("", function start(set) {
  var loc = window.location, new_uri;
if (loc.protocol === "https:") {
    new_uri = "wss:";
} else {
    new_uri = "ws:";
}
new_uri += "//" + loc.host;
new_uri += loc.pathname + "/data/info";

  const socket = new WebSocket(new_uri);

  socket.addEventListener("message", function (event : MessageEvent<string>) {
    set(event.data.replaceAll('"', ""));
  });
  return function stop() {
    socket.close();
  };
});
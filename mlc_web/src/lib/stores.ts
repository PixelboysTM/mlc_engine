import { readable, writable, type Readable, type Writable } from "svelte/store";

export function make_ws_uri(path: string): string {
  var loc = window.location, new_uri;
  if(loc.protocol === "https:") {
    new_uri = "wss:";
  } else {
    new_uri = "ws:";
  }
  new_uri += "//" + loc.host;
  new_uri += loc.pathname + path;
  return new_uri;
}

export type InfoKind = "None" | "FixtureTypesUpdated" | "ProjectSaved" | "ProjectLoaded" | "SystemShutdown" | "UniversesUpdated" | {"UniversePatchChanged": number} |"EffectListChanged";
export const info: Readable<InfoKind> = readable("None", function start(set) {

var new_uri = make_ws_uri("/data/info");

  const socket = new WebSocket(new_uri);

  socket.addEventListener("message", function (event : MessageEvent<string>) {
    set(JSON.parse(event.data));
  });
  return function stop() {
    socket.close();
  };
});

function createToast() {
  const {subscribe, set, update}: Writable<ToastNotification | null> = writable(null);
  const history: Writable<ToastNotification[]> = writable([]);

  return {
    subscribe,
    reset: () => set(null),
    push: (e: ToastNotification) => {update((n) => e); history.update((n) => [...n, e])},
  }
}

export const toastNotifier = createToast();

export type ToastNotification = {
  level: "info" | "warning" | "error",
  title: string,
  msg: string
}
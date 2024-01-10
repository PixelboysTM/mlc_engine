import { readable, writable, type Readable, type Writable } from "svelte/store";

export type InfoKind = "None" | "FixtureTypesUpdated" | "ProjectSaved" | "ProjectLoaded" | "SystemShutdown" | "UniversesUpdated" | {"UniversePatchChanged": number}; 
export const info: Readable<InfoKind> = readable("None", function start(set) {
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
    set(JSON.parse(event.data));
  });
  return function stop() {
    socket.close();
  };
});

function createToast() {
  const {subscribe, set, update}: Writable<ToastNotification[]> = writable([]);
  const history: Writable<ToastNotification[]> = writable([]);

  return {
    subscribe,
    reset: () => set([]),
    push: (e: ToastNotification) => {update((n) => [...n, e]); history.update((n) => [...n, e])},
    pull: () => {
      let element: null | ToastNotification = null;
      update((n) => {
        if(n.length > 0){
          element = n[0];
        }
        return [...n.slice(1)];
      })

      return element;
    }
  }
}

export const toastNotifier = createToast();

export type ToastNotification = {
  level: "info" | "warning" | "error",
  title: string,
  msg: string
}
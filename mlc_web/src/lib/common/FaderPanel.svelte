<script lang="ts">
  import Fader from "./Fader.svelte";
  import { info } from "../stores";

  let values: number[] = [];
  for (let i = 0; i < 512; i++) {
    values.push(0);
  }
  let currentUniverse = 0;

  let universes: number[] = [];
  fetch("/data/universes").then((body) =>
    body.json().then((json) => {
      universes = json;
      currentUniverse = universes.at(0) ?? 1;
      setUniverse(currentUniverse);
    })
  );

  info.subscribe((data) => {
    if (data == "UniversesUpdated") {
      fetch("/data/universes").then((body) =>
        body.json().then((json) => {
          universes = json;
          currentUniverse = universes.at(0) ?? 1;
        })
      );
    }
  });

  function makeName(t: number) {
    let name = "";
    if (t < 10) {
      name = "00" + t;
    } else if (t < 100) {
      name = "0" + t;
    } else {
      name = "" + t;
    }
    return name;
  }

  type RuntimeValueUpdate =
    | {
        ValueUpdated: {
          universe: number;
          channel_index: number;
          value: number;
        };
      }
    | {
        Universe: {
          values: number[];
          universe: number;
        };
      };
  function getValuesWs() {
    var loc = window.location,
      new_uri;
    if (loc.protocol === "https:") {
      new_uri = "wss:";
    } else {
      new_uri = "ws:";
    }
    new_uri += "//" + loc.host;
    new_uri += loc.pathname + "/runtime/fader-values/get";

    const socket = new WebSocket(new_uri);
    socket.addEventListener("message", function (event: MessageEvent<string>) {
      let data = JSON.parse(event.data) as RuntimeValueUpdate;
      if (
        typeof data === "object" &&
        "ValueUpdated" in data &&
        data.ValueUpdated.universe == currentUniverse
      ) {
        values[data.ValueUpdated.channel_index] = data.ValueUpdated.value;
      } else if (
        typeof data === "object" &&
        "Universe" in data &&
        data.Universe.universe == currentUniverse
      ) {
        values = data.Universe.values;
      }
    });

    return socket;
  }

  function setValuesWs() {
    var loc = window.location,
      new_uri;
    if (loc.protocol === "https:") {
      new_uri = "wss:";
    } else {
      new_uri = "ws:";
    }
    new_uri += "//" + loc.host;
    new_uri += loc.pathname + "/runtime/fader-values/set";

    const socket = new WebSocket(new_uri);

    return socket;
  }

  let getSock = getValuesWs();
  let setSock = setValuesWs();

  function setBinding(channel: number, value: CustomEvent<number>) {
    let json = JSON.stringify({
      universe: currentUniverse,
      channel: channel,
      value: value.detail,
    });
    setSock.send(json);
  }

  function setUniverse(id: number) {
    currentUniverse = id;
    getSock.send(JSON.stringify(id));
  }
</script>

<div class="sliders">
  <div class="universe-list">
    {#each universes as universe}
      <div
        class="tab {universe === currentUniverse ? 'selected' : ''}"
        on:click={() => setUniverse(universe)}
        role="button"
        tabindex={0}
        on:keypress
      >
        {universe}
      </div>
    {/each}
  </div>
  <div class="faders">
    {#each values as value, i}
      <Fader {value} on:set={(v) => setBinding(i, v)} name={makeName(i + 1)}
      ></Fader>
    {/each}
  </div>
</div>

<style>
  .faders {
    display: grid;
    grid-template-columns: repeat(512, 1fr);
    grid-template-rows: 1fr;
    gap: 0.25rem;
    min-height: 0;
    min-width: 0;
    height: calc(100%);
    width: 100%;
    overflow-x: auto;
    overflow-y: hidden;
  }
  .sliders {
    height: calc(100% - 2rem);
    display: flex;
    gap: 0rem;
  }

  /* .slider {
    -webkit-appearance: slider-vertical;
    appearance: slider-vertical;
    width: 100%;
    height: fit-content;
    padding: 0 5px;
  } */

  .universe-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    height: 100%;
    width: 2rem;
    overflow-y: auto;
    border-right: #ff3e3e 1px solid;
  }

  .tab {
    width: 2rem;
    height: 3rem;
    background-color: #333;
    color: #fff;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 0.5rem 0 0 0.5rem;
    cursor: pointer;
  }
  .selected {
    background-color: #ff3e3e;
    color: #333;
  }
</style>

<script lang="ts">
  import type { UIEventHandler } from "svelte/elements";
  import Fader from "./Fader.svelte";

  let values: number[] = [];
  for (let i = 0; i < 512; i++) {
    let x = Math.floor(Math.random() * 255);
    values.push(x);
  }
  let currentUniverse = 0;

  let universes: number[] = [];
  fetch("/data/universes").then(body => body.json().then(json => {
    universes = json;
    currentUniverse = universes.at(0) ?? 1;
  }));

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

</script>

<div class="sliders" >
  <div class="universe-list">
    {#each universes as universe}
      <button>{universe}</button>
      {/each}
  </div>
  <div class="faders">
  {#each values as value, i}
    <Fader {value} name={makeName(i + 1)}></Fader>
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
    gap: 0.5rem;
  }

  /* .slider {
    -webkit-appearance: slider-vertical;
    appearance: slider-vertical;
    width: 100%;
    height: fit-content;
    padding: 0 5px;
  } */
</style>

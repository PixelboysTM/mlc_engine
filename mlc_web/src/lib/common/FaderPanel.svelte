<script lang="ts">
  import Fader from "./Fader.svelte";

  let values: number[] = [];
  for (let i = 0; i < 512; i++) {
    let x = Math.floor(Math.random() * 255);
    values.push(x);
  }

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

<div class="sliders">
  {#each values as value, i}
    <!-- <input
      type="range"
      min="0"
      max="255"
      bind:value={values[i]}
      class="slider"
      orient="vertical"
    /> -->
    <Fader {value} name={makeName(i)}></Fader>
  {/each}
</div>

<style>
  .sliders {
    display: grid;
    grid-template-columns: repeat(512, 1fr);
    grid-template-rows: 1fr;
    gap: 0.25rem;
    min-height: 0;
    min-width: 0;
    height: calc(100% - 2rem);
    width: 100%;
    overflow-x: auto;
  }

  /* .slider {
    -webkit-appearance: slider-vertical;
    appearance: slider-vertical;
    width: 100%;
    height: fit-content;
    padding: 0 5px;
  } */
</style>

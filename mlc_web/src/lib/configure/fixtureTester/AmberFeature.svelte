<script lang="ts">
  import { createSlider, melt } from "@melt-ui/svelte";
  import { createEventDispatcher } from "svelte";

  const dispatcher = createEventDispatcher<Record<string, number>>();

  const {
    elements: { root, range, thumbs },
    states: { value },
  } = createSlider({
    defaultValue: [0],
    max: 100,
    orientation: "vertical",
  });

  value.subscribe((data) => {
    dispatcher("value", data[0] / 100);
  });
</script>

<h3>Amber</h3>
<span class="root" use:melt={$root}>
  <span class="outer">
    <span class="range" use:melt={$range}></span>
  </span>
  <span class="thumb" use:melt={$thumbs[0]}></span>
</span>
<p class="value">{$value[0]}%</p>

<style>
  .root {
    position: relative;
    display: flex;
    width: 1.5rem;
    height: 100%;
    min-height: 10rem;
    max-height: 20rem;
    overflow: hidden;
    margin-left: auto;
    margin-right: auto;
    border: 1px solid var(--color-accent);
  }

  .outer {
    width: 100%;
    height: 100%;
    background-color: #000000;
  }

  .range {
    width: 100%;
    background-color: #ffbf00;
  }

  .thumb {
    width: 1rem;
    height: 0rem;
    border-radius: 50%;
    background-color: azure;
  }

  .value {
    margin-left: auto;
    margin-right: auto;
    margin-bottom: 0.2rem;
    margin-top: 0.2rem;
    text-align: center;
  }

  h3 {
    margin: 0;
    margin-bottom: 0.25rem;
  }
</style>

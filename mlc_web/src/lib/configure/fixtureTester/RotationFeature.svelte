<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Knob from "svelte-knob";
  import { writable } from "svelte/store";

  const dispatcher = createEventDispatcher<Record<string, number>>();

  //   value.subscribe((data) => {
  //     dispatcher("value", data[0] / 100);
  //   });

  let speed = writable(0);

  speed.subscribe((data) => {
    dispatcher("value", data / 100.0);
  });
</script>

<h3>Rotation Speed</h3>
<div class="dark">
  <Knob
    bind:value={$speed}
    max={100}
    min={-100}
    step={1}
    primaryColor={"#ff3e3e"}
    secondaryColor={"#333333"}
    textColor={"#fff"}
  ></Knob>
</div>
<p class="value">{$speed > 0 ? "Cw" : "Ccw"}</p>

<style>
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
    text-align: center;
  }

  .dark {
    margin-right: auto;
    margin-left: auto;
    width: max-content;
  }
</style>

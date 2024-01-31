<script lang="ts">
  import { createSlider, melt } from "@melt-ui/svelte";
  import { createEventDispatcher } from "svelte";
  import ColorPicker, { type RgbaColor } from "svelte-awesome-color-picker";

  const dispatcher = createEventDispatcher<Record<string, number[]>>();

  //   value.subscribe((data) => {
  //     dispatcher("value", data[0] / 100);
  //   });

  let hex = "#000000";

  function make_value(val: RgbaColor): number[] {
    return [val.r / 255, val.g / 255, val.b / 255];
  }
</script>

<h3>Rgb</h3>
<div class="dark">
  <ColorPicker
    on:input={(v) => {
      dispatcher(
        "value",
        make_value(v.detail.rgb ?? { r: 0, g: 0, b: 0, a: 0 })
      );
    }}
    bind:hex
    isDialog={false}
    isDark={true}
    isAlpha={false}
  ></ColorPicker>
</div>
<p class="value">{hex}</p>

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
    --cp-bg-color: var(--color-background);
    --cp-border-color: var(--color-accent);
    --cp-input-color: var(--color-panel);
    --cp-button-hover-color: #777;
    --slider-width: 1.25rem;
    --picker-indicator-size: 0.75rem;
  }
</style>

<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";
  import { make_ws_uri } from "../../stores";
  import { Lamp, X } from "lucide-svelte";
  import DimmerFeature from "./DimmerFeature.svelte";
  import RgbFeature from "./RgbFeature.svelte";
  import WhiteFeature from "./WhiteFeature.svelte";
  import RotationFeature from "./RotationFeature.svelte";
  import DualRingSpinner from "../../misc/DualRingSpinner.svelte";
  import PanTiltFeature from "./PanTiltFeature.svelte";
  import AmberFeature from "./AmberFeature.svelte";

  export let id: string;
  export let name: string;

  type FeatureKind =
    | "Dimmer"
    | "Rgb"
    | "White"
    | "Rotation"
    | "PanTilt"
    | "Amber";

  var features: FeatureKind[] = [];

  const socket = new WebSocket(make_ws_uri("/runtime/feature/" + id));
  socket.addEventListener("message", (event) => {
    features = JSON.parse(event.data);
  });

  onDestroy(() => {
    socket.close();
  });

  const dispatcher = createEventDispatcher();

  socket.onopen = () => {
    socket.send('"GetAvailableFeatures"');
  };
</script>

<div class="backdrop">
  <div class="panel">
    <div class="header">
      <div class="icon">
        <Lamp size="2rem" color={"var(--color-accent)"} />
      </div>
      <h3>{name}</h3>
      <button class="close" on:click={() => dispatcher("close")}
        ><X size={"1.25rem"} /></button
      >
    </div>
    <div class="body">
      {#if features.length === 0}
        <DualRingSpinner></DualRingSpinner>
      {/if}
      {#each features as feature}
        <div class="feature">
          {#if feature === "Dimmer"}
            <DimmerFeature
              on:value={(v) =>
                socket.send('{"Dimmer": {"value": ' + v.detail + "}}")}
            ></DimmerFeature>
          {:else if feature === "Rgb"}
            <RgbFeature
              on:value={(v) =>
                socket.send(
                  '{"Rgb": {"red": ' +
                    v.detail[0] +
                    ', "green": ' +
                    v.detail[1] +
                    ', "blue":' +
                    v.detail[2] +
                    "}}"
                )}
            ></RgbFeature>
          {:else if feature === "White"}
            <WhiteFeature
              on:value={(v) =>
                socket.send('{"White": {"value": ' + v.detail + "}}")}
            ></WhiteFeature>
          {:else if feature === "Rotation"}
            <RotationFeature
              on:value={(v) =>
                socket.send('{"Rotation": { "value":' + v.detail + "}}")}
            ></RotationFeature>
          {:else if feature === "PanTilt"}
            <PanTiltFeature
              on:value={(v) =>
                socket.send(
                  '{"PanTilt": { "pan": ' +
                    v.detail[0] +
                    ', "tilt":' +
                    v.detail[1] +
                    "}}"
                )}
            ></PanTiltFeature>
          {:else if feature === "Amber"}
            <AmberFeature
              on:value={(v) =>
                socket.send('{"Amber": {"value": ' + v.detail + "}}")}
            ></AmberFeature>
          {:else}
            <h3>Unknown Feature: {feature}</h3>
          {/if}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100vw;
    height: 100vh;

    background-color: var(--color-background-transparent);

    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 50;
  }
  .panel {
    width: 60vw;
    height: 80vh;
    background-color: var(--color-panel);
    border-radius: var(--number-border-radius);
    display: flex;
    flex-direction: column;
    z-index: 51;
  }

  .header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
    padding-top: 0.25rem;
    padding-bottom: 0.25rem;
    border-bottom: var(--color-accent) 1px solid;
  }

  .header h3 {
    margin: 0;
    padding: 0;
  }

  .body {
    height: 100%;
    margin: 0.5rem;
    overflow-x: auto;
    display: flex;
    flex-direction: row;
    gap: 0.5rem;
  }

  .feature {
    background-color: #1a1a1a;
    border-radius: var(--number-border-radius);
    height: max-content;
    padding: 0.5rem;
  }

  .icon {
    padding-right: 0.25rem;
    border-right: var(--color-accent) 1px solid;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
  }

  .close {
    border: none;
    color: var(--color-text);
    font-size: 1rem;
    cursor: pointer;
    padding: 0.5rem 0.5rem;
    border-radius: var(--number-border-radius);
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .close:hover {
    color: var(--color-accent);
  }
</style>

<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";
  import { make_ws_uri } from "../../stores";
  import Icon from "svelte-icons-pack/Icon.svelte";
  import BsLampFill from "svelte-icons-pack/bs/BsLampFill";
  import DimmerFeature from "./DimmerFeature.svelte";
  import RgbFeature from "./RgbFeature.svelte";

  export let id: string;
  export let name: string;

  type FeatureKind = "Dimmer" | "Rgb";

  var features: FeatureKind[] = [];

  const socket = new WebSocket(make_ws_uri("/runtime/feature/" + id));
  socket.addEventListener("message", (event) => {
    features = JSON.parse(event.data);
    console.log(event.data);
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
        <Icon size="2rem" color={"#ff3e3e"} src={BsLampFill}></Icon>
      </div>
      <h3>{name}</h3>
      <button on:click={() => dispatcher("close")}>X</button>
    </div>
    <div class="body">
      {#each features as feature}
        <div class="feature">
          {#if feature == "Dimmer"}
            <DimmerFeature
              on:value={(v) =>
                socket.send('{"Dimmer": {"value": ' + v.detail + "}}")}
            ></DimmerFeature>
          {:else if feature == "Rgb"}
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

    background-color: #000000e5;

    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 50;
  }
  .panel {
    width: 60vw;
    height: 80vh;
    background-color: #151111;
    border-radius: 0.25rem;
    display: flex;
    flex-direction: column;
    z-index: 51;
  }

  .header {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    border-bottom: #ff3e3e 1px solid;
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
    border-radius: 0.25rem;
    height: max-content;
    padding: 0.5rem;
  }

  .icon {
    padding-right: 0.25rem;
    border-right: #ff3e3e 1px solid;
  }
</style>

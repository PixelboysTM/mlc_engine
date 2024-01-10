<script lang="ts">
  import type { FixtureUniverse } from "fixture-types";
  import DualRingSpinner from "../misc/DualRingSpinner.svelte";
  import { info } from "../stores";
  let promise: Promise<number[]>;
  getUniverses();

  function getUniverses() {
    promise = fetch("/data/universes").then((res) => res.json());
  }

  info.subscribe((data) => {
    if (
      typeof data === "object" &&
      "UniversePatchChanged" in data &&
      data.UniversePatchChanged == selectedUniverse
    ) {
      setUniverse(selectedUniverse, true);
    }
    if (data == "UniversesUpdated") {
      getUniverses();
    }
  });
  function setUniverse(id: number, force: boolean = false) {
    if (id == selectedUniverse && !force) return;
    selectedUniverse = id;
    currentUniverse = fetch(`/data/universes/${id}`).then((res) => {
      console.log(res);
      return res.json();
    });
  }

  let selectedUniverse: number = 1;
  let currentUniverse: Promise<FixtureUniverse> = fetch(
    `/data/universes/${selectedUniverse}`
  ).then((res) => res.json());

  function getPatchedClass(num_channels: number, i: number) {
    if (i == 0) {
      if (num_channels == 1) {
        return "start end";
      } else {
        return "start";
      }
    } else if (i == num_channels - 1) {
      return "end";
    } else {
      return "middle";
    }
  }
</script>

{#await promise}
  <DualRingSpinner />
{:then universes}
  <div class="explorer">
    <div class="tabs">
      {#each universes as universe}
        <div
          class="tab {universe === selectedUniverse ? 'selected' : ''}"
          on:click={() => setUniverse(universe)}
          role="button"
          tabindex={0}
          on:keypress
        >
          {universe}
        </div>
      {/each}
    </div>
    <div class="universe">
      {#await currentUniverse}
        <div class="center">
          <DualRingSpinner />
        </div>
      {:then data}
        <div class="channels">
          {#each data.channels as channel, i}
            {#if channel != undefined}
              <div
                data-tooltip={data.fixtures[channel.fixture_index].name}
                class="patched-channel {getPatchedClass(
                  data.fixtures[channel.fixture_index].num_channels,
                  channel.channel_index
                )}"
              >
                {#if channel.channel_index == 0}
                  <code
                    >{data.fixtures[channel.fixture_index].start_channel}</code
                  >
                {/if}
              </div>
            {:else}
              <div class="channel">
                <code>{i}</code>
              </div>
            {/if}
          {/each}
        </div>
      {:catch error}
        <p>Error loading universe</p>
      {/await}
    </div>
  </div>
{:catch error}
  <p>Error loading universes</p>
{/await}

<style>
  [data-tooltip] {
    position: relative;
    cursor: help;
  }

  [data-tooltip]::after {
    position: absolute;
    opacity: 0;
    pointer-events: none;
    content: attr(data-tooltip);
    left: 0;
    top: calc(100% + 10px);
    border-radius: 3px;
    box-shadow: 0 0 5px 2px rgba(100, 100, 100, 0.6);
    background-color: #151111;
    z-index: 10;
    padding: 8px;
    width: 15rem;
    color: #fff;
    /* transform: translateY(-20px);
    transition: all 150ms cubic-bezier(0.25, 0.8, 0.25, 1); */
  }

  [data-tooltip]:hover::after {
    opacity: 1;
    transform: translateY(0);
    /* transition-duration: 300ms; */
  }
  .explorer {
    width: 100%;
    display: flex;
    flex-direction: column;
  }
  .tabs {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 2rem;
    border-bottom: #ff3e3e 1px solid;
  }
  .tab {
    width: 3rem;
    height: 100%;
    background-color: #333;
    color: #fff;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 0.5rem 0.5rem 0 0;
    cursor: pointer;
  }
  .selected {
    background-color: #ff3e3e;
    color: #333;
  }
  .channels {
    display: grid;
    grid-template-columns: repeat(32, 1fr);
    border: #ff3e3e 1px solid;
    overflow: hidden;
  }
  .channel {
    width: 100%;
    height: 2.5rem;
    background-color: #333;
    color: #fff;
    display: flex;
    justify-content: center;
    align-items: center;
    border: #151111 1px solid;
  }

  .patched-channel {
    width: 100%;
    height: 2.5rem;
    background: linear-gradient(0deg, #3eff41 0%, #3e88ff 100%);
    color: #333;
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .patched-channel.start {
    border-left: #333 1px solid;
    border-top: #333 1px solid;
    border-bottom: #333 1px solid;
  }
  .patched-channel.end {
    border-right: #333 1px solid;
    border-top: #333 1px solid;
    border-bottom: #333 1px solid;
  }
  .patched-channel.middle {
    border-top: #333 1px solid;
    border-bottom: #333 1px solid;
  }

  .universe {
    width: 100%;
    overflow-y: auto;
  }
  .center {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>

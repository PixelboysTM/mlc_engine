<script lang="ts">
  import { createPopover, melt } from "@melt-ui/svelte";
  import {
    Plus,
    Play,
    Rewind,
    FastForward,
    ZoomIn,
    ZoomOut,
    X,
  } from "lucide-svelte";
  import { fade } from "svelte/transition";
  import type { Effect, Track } from "../../../customTypings/Effect";

  export let effect: Effect | undefined = undefined;

  let tracks: Track[] = [];

  $: tracks = effect?.tracks ?? [];

  const {
    elements: { trigger, content, arrow, close },
    states: { open },
  } = createPopover({ forceVisible: true });

  function createFaderTrack() {
    console.log(effect);
    if (effect == undefined) return;
    effect.tracks.push({
      FaderTrack: {
        address: { address: 0, universe: 0 },
        values: [],
      },
    });
    effect = effect;
  }
</script>

<div class="timeline">
  <div class="toolbar">
    <div class="left">
      <div class="iconBtn" use:melt={$trigger}><Plus size="1rem" /></div>
    </div>
    <div class="middle">
      <div class="iconBtn"><Rewind size="1rem" /></div>
      <div class="iconBtn"><Play size="1rem" /></div>
      <div class="iconBtn"><FastForward size="1rem" /></div>
    </div>
    <div class="right">
      <div class="iconBtn"><ZoomOut size="1rem" /></div>
      <div class="iconBtn"><ZoomIn size="1rem" /></div>
    </div>
  </div>
  <div class="tracks">
    <div class="time">Time</div>
    {#each tracks as track}
      <div class="track">
        {#if "FaderTrack" in track}
          <div class="faderTrack">
            <p>Fader Track</p>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<!-- Popover -->
{#if $open}
  <div class="content" use:melt={$content} transition:fade={{ duration: 100 }}>
    <span use:melt={$arrow} />
    <div class="popover">
      <p>Create New Track</p>

      <button on:click={() => createFaderTrack()}>Fader Track</button>
    </div>
    <button use:melt={$close} class="close">
      <X class="square"></X>
    </button>
  </div>
{/if}

<style>
  .timeline {
    width: 100%;
    height: 100%;
  }

  .toolbar {
    width: 100%;
    height: 2.5rem;
    /* background-color: #333; */
    border-bottom: 1px solid var(--color-accent);
    display: grid;
    grid-template-columns: auto auto auto;
  }

  .toolbar .left {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 0.25rem;
  }

  .toolbar .middle {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 0.25rem;
  }

  .toolbar .right {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 0.25rem;
  }

  .iconBtn {
    width: 1.5rem;
    height: 1.5rem;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    background-color: var(--color-button);
    border-radius: var(--number-border-radius);
    color: var(--color-text);
  }
  .iconBtn:hover {
    background-color: var(--color-accent);
  }

  /* Popover */
  .content {
    z-index: 50;
    width: 12rem;
    border-radius: var(--number-border-radius);
    background-color: var(--color-panel);
    padding: 0.5rem;
    box-shadow: 0 0 0.2rem 0.2rem #000000c4;
  }

  .popover {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .popover p {
    margin: 0;
  }

  .popInput {
    display: flex;
    height: 2rem;
    width: 100%;
    border-radius: 0;
    border: none;
    background-color: var(--color-panel);
    font-size: small;
    align-items: center;
    justify-items: center;
    border-left: 1px solid var(--color-background);
  }

  .popInput:focus-visible {
    outline: none;
  }

  .close {
    position: absolute;
    right: 0.5rem;
    top: 0.5rem;
    display: flex;
    height: 1.5rem;
    width: 1.5rem;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    padding: 0;
    color: var(--color-text);
  }
  .close:hover {
    color: var(--color-accent);
  }

  .square {
    width: 1.25rem;
    height: 1.25rem;
    cursor: pointer;

    color: var(--color-text);
  }

  .square:hover {
    color: var(--color-accent);
  }

  .tracks {
    width: 100%;
    height: calc(100% - 2.6rem);
    overflow-y: auto;
  }

  .time {
    position: sticky;
    top: 0;
  }
</style>

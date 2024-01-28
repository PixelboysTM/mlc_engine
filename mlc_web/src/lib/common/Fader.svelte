<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let value: number = 0;
  export let name: string = "";

  let isDown = false;
  let range: HTMLDivElement;
  function resize(e: MouseEvent) {
    if (!isDown) return;
    let cur = e.y;
    // console.log(e);
    // console.log(range);
    // console.log(range.getBoundingClientRect());
    let start = range.getBoundingClientRect().y;
    let height = range.getBoundingClientRect().height;

    let t = (1 - (cur - start) / height) * 255;
    value = Math.max(0, Math.min(255, Math.floor(t)));
    dispatchSet();
  }

  const dispatch = createEventDispatcher();
  function dispatchSet() {
    dispatch("set", value);
  }

  let isEdit = false;
  let popup: HTMLDivElement;
</script>

<svelte:body
  on:mousedown={(e) => {
    let box = popup.getBoundingClientRect();
    if (
      e.x < box.left ||
      e.x > box.right ||
      e.y < box.top ||
      e.y > box.bottom
    ) {
      isEdit = false;
    }
  }}
  on:mousemove={(e) => resize(e)}
  on:mouseup={(e) => {
    isDown = false;
  }}
/>

<div class="fader">
  <div class="fader__name">{name}</div>
  <div class="range" bind:this={range}>
    <div
      class="filler"
      style="height: {(1 - value / 255) * 100}%;"
      on:mousedown={(e) => (isDown = e.button === 0)}
      on:mouseup={(e) => (isDown = e.button === 0)}
      on:mousemove={(e) => resize(e)}
      role="slider"
      aria-valuenow={value}
      aria-valuemin={0}
      aria-valuemax={255}
      tabindex={0}
    />
    <div
      class="inner"
      style="height: {(value / 255) * 100}%;"
      on:mousedown={(e) => (isDown = e.button === 0)}
      on:mousemove={(e) => resize(e)}
      on:mouseup={(e) => (isDown = e.button === 0)}
      role="slider"
      aria-valuenow={value}
      aria-valuemin={0}
      aria-valuemax={255}
      tabindex={0}
    />
  </div>
  <div
    class="fader__value"
    on:click={(e) => {
      isEdit = isEdit || e.button === 0;
      e.preventDefault();
    }}
    role="button"
    tabindex={1}
    on:keypress
  >
    <p>{value}</p>
  </div>
  {#if isEdit}
    <div
      class="fader-popup"
      bind:this={popup}
      style="top: {range.getBoundingClientRect()
        .top}px; left: {range.getBoundingClientRect().left}px;"
    >
      <input
        type="number"
        on:change={() => dispatchSet()}
        bind:value
        min={0}
        max={255}
      />
    </div>
  {/if}
</div>

<style>
  input[type="number"] {
    width: 100%;
  }

  .fader-popup {
    position: absolute;
    width: 3rem;
  }

  .fader {
    display: grid;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    grid-template-rows: 1.5rem 1fr 1.5rem;
    height: 100%;
    min-height: 0;
    min-width: 0;
    text-align: center;
    width: 1.5rem;
    background-color: #242424;
    border-radius: 0.25rem;
  }

  .fader__name {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-text);
    margin-bottom: 0.1rem;
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }

  .range {
    width: 100%;
    height: 100%;
    overflow: hidden;
    border-top: #151111 1px solid;
    border-bottom: #151111 1px solid;
  }

  .filler {
    width: 100%;
    cursor: ns-resize;
  }
  .inner {
    width: 100%;
    background: linear-gradient(0deg, #fff 0%, #ff3e3e 100%);
    cursor: ns-resize;
  }

  .fader__value {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-text);
    margin-top: 0.1rem;
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }
</style>

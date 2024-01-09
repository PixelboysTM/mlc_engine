<script lang="ts">
  export let value: number = 0;
  export let name: string = "";
</script>

<div class="fader">
  <div class="fader__name">{name}</div>
  <div class="range">
    <div class="filler" style="height: {(1 - value / 255) * 100}%;"></div>
    <div class="inner" style="height: {(value / 255) * 100}%;"></div>
  </div>
  <div class="fader__value" contenteditable="true" bind:innerText={value}>
    {value}
  </div>
</div>

<style>
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
  }
  .inner {
    width: 100%;
    background-color: green;
  }

  .fader__value {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-text);
    margin-top: 0.1rem;
  }
</style>

<script lang="ts">
  import { createDialog, melt } from "@melt-ui/svelte";
  import { back } from "@melt-ui/svelte/internal/helpers";
  import type { EndpointMappingConfig } from "../../customTypings/EndpointMapping";
  import { get } from "svelte/store";
  import DualRingSpinner from "../misc/DualRingSpinner.svelte";

  const {
    elements: {
      trigger,
      overlay,
      content,
      title,
      description,
      close,
      portalled,
    },
    states: { open },
  } = createDialog();

  let promise: Promise<EndpointMappingConfig> = getMappping();

  function getMappping(): Promise<EndpointMappingConfig> {
    return fetch("/runtime/endpoints/get").then((res) => res.json());
  }

  open.subscribe((value) => {
    if (value) {
      promise = getMappping();
    }
  });
</script>

<button use:melt={$trigger} class="trigger">Endpoints</button>
<div use:melt={$portalled}>
  {#if open}
    <div use:melt={$overlay} class="overlay"></div>
    <div use:melt={$content} class="content">
      <div class="header">
        <h3 use:melt={$title} class="title">Endpoints</h3>
        <button use:melt={$close} class="close">X</button>
      </div>
      <div class="body">
        {#await promise}
          <div class="center">
            <DualRingSpinner></DualRingSpinner>
          </div>
        {:then mapping}
          <pre>{JSON.stringify(mapping, null, 2)}</pre>
        {:catch error}
          <p>{error}</p>
        {/await}
      </div>
    </div>
  {/if}
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    z-index: 40;

    background-color: rgba(0, 0, 0, 0.177);
  }

  .content {
    position: fixed;
    top: 50%;
    left: 50%;

    z-index: 50;

    max-height: 85vh;
    max-width: 90vw;

    transform: translate(-50%, -50%);

    border-radius: 0.5rem;

    background-color: #151111;
    padding: 1rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .close {
    display: inline-flex;
    align-items: center;
    justify-content: center;

    position: absolute;
    right: 10px;
    top: 10px;

    appearance: none;

    height: 1.5rem;
    width: 1.5rem;

    border-radius: 9999px;

    color: #fff;
    background-color: transparent;
  }

  .close:hover {
    color: #ff3e3e;
    border: none;
  }

  .close:focus {
    color: #ff3e3e;
    border: none;
    outline: none;
    border: none;
  }

  .title {
    margin: 0;

    font-size: 1.125rem;
    line-height: 1.75rem;
    font-weight: 500;
  }
</style>

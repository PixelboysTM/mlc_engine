<script lang="ts">
  import { createDialog, melt } from "@melt-ui/svelte";
  import type { EndpointMappingConfig } from "../../customTypings/EndpointMapping";
  import DualRingSpinner from "../misc/DualRingSpinner.svelte";

  const {
    elements: { trigger, overlay, content, title, close, portalled },
    states: { open },
  } = createDialog();

  let promise: Promise<EndpointMappingConfig> = getMappping();
  let availUniverses: string[] = [];

  function getMappping(): Promise<EndpointMappingConfig> {
    return fetch("/runtime/endpoints/get").then((res) => res.json());
  }

  open.subscribe((value) => {
    if (value) {
      promise = getMappping();
      promise.then((mapping) => {
        fetch("/data/universes")
          .then((res) => res.json())
          .then((data) => {
            availUniverses = data.map((v: number) => v.toString());
          });
      });
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
          <div class="mappings">
            {#each availUniverses as u}
              <div class="mapping">
                <p>{u}</p>
                {#each mapping.endpoints[u] as e}
                  {#if "Sacn" in e}
                    <p>
                      Sacn: Universe ({e.Sacn.universe}) Speed ({e.Sacn.speed})
                    </p>
                  {/if}
                {/each}
              </div>
            {/each}
          </div>
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

    background-color: var(--color-background-transparent);
  }

  .content {
    position: fixed;
    top: 50%;
    left: 50%;

    z-index: 50;

    max-height: 85vh;
    max-width: 90vw;

    min-width: 80vw;
    min-height: 80vh;

    transform: translate(-50%, -50%);

    border-radius: var(--number-border-radius);

    background-color: var(--color-panel);
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

    color: var(--color-text);
    /* background-color: transparent; */
  }

  .close:hover {
    color: var(--color-accent);
    border: none;
  }

  /* .close:focus {
    color: var(--color-accent);
    border: none;
    outline: none;
    border: none;
  } */

  .title {
    margin: 0;

    font-size: 1.125rem;
    line-height: 1.75rem;
    font-weight: 500;
  }
</style>

<script lang="ts">
  import DualRingSpinner from "../misc/DualRingSpinner.svelte";
  import { toastNotifier } from "../stores";

  type Settings = {
    save_on_quit: boolean;
  };
  let settings: Settings | undefined = undefined;
  fetch("/settings/get")
    .then((res) => res.json())
    .then((data) => {
      settings = data;
    });

  function updateSettings() {
    fetch("/settings/update", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(settings),
    }).then((r) => (hasChanges = false));
  }

  function openEndpointMapping() {
    toastNotifier.push({
      level: "info",
      msg: "Not implemented yet",
      title: "Info",
    });
  }

  let hasChanges: boolean = false;
  function handleChanges() {
    hasChanges = true;
  }
</script>

<div class="settings-panel">
  <div class="settings">
    {#if settings == undefined}
      <DualRingSpinner />
    {:else}
      {#if hasChanges}
        <div class="unsaved">
          <p>Unsaved changes press Update to Confirm.</p>
        </div>
      {/if}
      <div class="setting">
        <p>Save on quit</p>
        <input
          type="checkbox"
          on:change={handleChanges}
          bind:checked={settings.save_on_quit}
        />
      </div>
    {/if}
  </div>
  <div class="btns">
    <button
      type="button"
      disabled={!hasChanges}
      on:click={() => updateSettings()}>Update</button
    >
    <button type="button" on:click={() => openEndpointMapping()}
      >Endpoints</button
    >
  </div>
</div>

<style>
  .settings-panel {
    width: 100%;
    height: calc(100% - 2rem);
    display: grid;
    grid-template-rows: 1fr 3rem;
  }
  .settings {
    width: 100%;
    height: 5rem;
    display: flex;
    flex-direction: column;
  }
  .setting {
    display: grid;
    grid-template-columns: 1fr 1fr;
    align-items: center;
    justify-content: space-between;
  }

  .unsaved {
    width: 100%;
    height: 5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: #ffa83e;
  }
</style>

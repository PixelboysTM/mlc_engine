<script lang="ts">
  import UploadFixture from "./configure/UploadFixture.svelte";
  import BsCloudUploadFill from "svelte-icons-pack/bs/BsCloudUploadFill";
  import FiSave from "svelte-icons-pack/fi/FiSave";
  import BsGearFill from "svelte-icons-pack/bs/BsGearFill";
  import BsPencilFill from "svelte-icons-pack/bs/BsPencilFill";
  import BsLightbulbFill from "svelte-icons-pack/bs/BsLightbulbFill";
  import marvin from "../assets/icon.png";
  import Icon from "svelte-icons-pack/Icon.svelte";

  let showUpload = false;

  export let pane: "configure" | "program" | "show" = "configure";
</script>

<div>
  <img class="iconMarvin" src={marvin} alt="MLC" />

  <div class="tabs">
    <button
      class="icon configure {pane == 'configure' ? 'selected' : ''}"
      title="Configure"
      on:click={() => (pane = "configure")}><Icon src={BsGearFill} /></button
    >
    <button
      class="icon program {pane == 'program' ? 'selected' : ''}"
      title="Program"
      on:click={() => (pane = "program")}><Icon src={BsPencilFill} /></button
    >
    <button
      class="icon show {pane == 'show' ? 'selected' : ''}"
      title="Show"
      on:click={() => (pane = "show")}><Icon src={BsLightbulbFill} /></button
    >
  </div>
  <div class="tabs right">
    {#if pane == "configure"}
      <button
        title="Upload Fixture"
        class="icon"
        on:click={() => (showUpload = true)}
        ><Icon src={BsCloudUploadFill} /></button
      >
    {/if}
    <button
      title="Save Project"
      class="icon"
      on:click={() => fetch("/data/save")}><Icon src={FiSave} /></button
    >
  </div>

  {#if showUpload}
    <UploadFixture on:close={() => (showUpload = false)} />
  {/if}
</div>

<style>
  div {
    width: 100%;
    height: 3rem;
    background-color: var(--color-panel);
    display: grid;
    grid-template-columns: 1fr 5fr 1fr;
  }
  div.tabs {
    display: flex;
    place-content: center;
    align-items: center;
    width: 100%;
  }

  .iconMarvin {
    width: 3rem;
    height: 80%;
    margin-left: 0.2rem;
    margin-top: 0.25rem;
    cursor: pointer;
  }

  .icon {
    width: auto;
    height: 80%;
    margin-left: 0.2rem;
    color: var(--color-text);
  }
  div.right {
    place-content: end;
  }
  .selected.configure {
    color: var(--color-accent);
  }
  .selected.program {
    color: var(--color-secondary);
  }
  .selected.show {
    color: var(--color-tertiary);
  }
</style>

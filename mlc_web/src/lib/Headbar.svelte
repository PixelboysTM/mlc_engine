<script lang="ts">
  import UploadFixture from "./configure/UploadFixture.svelte";
  import {
    Settings,
    Pencil,
    Lightbulb,
    UploadCloud,
    Save,
    ArrowUpRightFromSquare,
  } from "lucide-svelte";
  import marvin from "../assets/icon.png";

  let showUpload = false;

  type Tab = "configure" | "program" | "show";

  export let pane: Tab = "configure";

  function setTab(tab: Tab) {
    if (pane == tab) return;

    pane = tab;
    localStorage.setItem("lastTab", tab);
  }

  setTab((localStorage.getItem("lastTab") as Tab) ?? pane);
</script>

<div>
  <img class="iconMarvin" src={marvin} alt="MLC" />

  <div class="tabs">
    <button
      class="icon configure {pane === 'configure' ? 'selected' : ''}"
      title="Configure"
      on:click={() => setTab("configure")}><Settings size={"100%"} /></button
    >
    <button
      class="icon program {pane === 'program' ? 'selected' : ''}"
      title="Program"
      on:click={() => setTab("program")}><Pencil size={"100%"} /></button
    >
    <button
      class="icon show {pane === 'show' ? 'selected' : ''}"
      title="Show"
      on:click={() => setTab("show")}><Lightbulb size={"100%"} /></button
    >
  </div>
  <div class="tabs right">
    {#if pane === "configure"}
      <button
        title="Upload Fixture"
        class="icon"
        on:click={() => (showUpload = true)}
        ><UploadCloud size={"100%"} /></button
      >
    {/if}
    {#if pane === "program"}
      <button
        title="Open 3D Viewer"
        class="icon"
        on:click={() => {
          window.open("/viewer3d", "_blank");
        }}><ArrowUpRightFromSquare size={"100%"} /></button
      >
    {/if}
    <button
      title="Save Project"
      class="icon"
      on:click={() => fetch("/data/save")}><Save size={"100%"} /></button
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

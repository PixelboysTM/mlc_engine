<script lang="ts">
  import svelteLogo from "../assets/svelte.svg";
  import UploadFixture from "./UploadFixture.svelte";
  import viteLogo from "/vite.svg";
  // import FaUpload from "svelte-icons/fa/FaUpload.svelte";
  import BsCloudUploadFill from "svelte-icons-pack/bs/BsCloudUploadFill";
  import FiSave from "svelte-icons-pack/fi/FiSave";
  import BsGearFill from "svelte-icons-pack/bs/BsGearFill";
  // import BsPencilSquare from "svelte-icons-pack/bs/BsPencilSquare";
  import BsPencilFill from "svelte-icons-pack/bs/BsPencilFill";
  import BsLightbulbFill from "svelte-icons-pack/bs/BsLightbulbFill";
  import Icon from "svelte-icons-pack/Icon.svelte";

  let showUpload = false;

  export let pane: "configure" | "program" | "show" = "configure";
</script>

<div>
  <span
    ><a id="a" href="#top">M</a><a id="b" href="#top">L</a><a id="c" href="#top"
      >C</a
    ></span
  >
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
    <button
      title="Upload Fixture"
      class="icon"
      on:click={() => (showUpload = true)}
      ><Icon src={BsCloudUploadFill} /></button
    >
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
    background-color: #151111;
    display: grid;
    grid-template-columns: 1fr 5fr 1fr;
  }
  div.tabs {
    display: flex;
    place-content: center;
    align-items: center;
    width: 100%;
  }
  span a {
    font-size: 2rem;
    color: #fff;
    text-decoration: none;
    font-family: "Roboto Mono", monospace;
    font-weight: 700;
    padding: 0.5rem;
    margin: 0rem;
    border-radius: 0.5rem;
    transition: all 0.2s ease-in-out;
  }

  #a {
    color: #ff4e3e;
  }
  #b {
    color: #3eff41;
  }
  #c {
    color: #3e88ff;
  }

  #a:hover {
    background-color: #ffdc3e;
    color: #fff;
  }
  #b:hover {
    background-color: #3effe5;
    color: #fff;
  }
  #c:hover {
    background-color: #ff3ee8;
    color: #fff;
  }

  .icon {
    width: auto;
    height: 80%;
    margin-left: 0.2rem;
    color: #fff;
  }
  div.right {
    place-content: end;
  }
  .selected.configure {
    color: #ff3e3e;
  }
  .selected.program {
    color: #3eff41;
  }
  .selected.show {
    color: #3e88ff;
  }
</style>

<script lang="ts">
  import DisconnectHelper from "./lib/DisconnectHelper.svelte";
  import Headbar from "./lib/Headbar.svelte";
  import Toast from "./lib/Toast.svelte";
  import UniverseExplorer from "./lib/configure/UniverseExplorer.svelte";
  import ConfigurePanel from "./lib/configure/ConfigurePanel.svelte";
  import { info, toastNotifier } from "./lib/stores";
  import ProgramPanel from "./lib/program/ProgramPanel.svelte";

  let promise = getFixtureTypes();

  async function getFixtureTypes() {
    const response = await fetch("/data/get/fixture-types");
    const data = await response.json();
    return data;
  }

  let s = info.subscribe((data) => {
    console.log(data);
    if (data == "FixtureTypesUpdated") {
      promise = getFixtureTypes();
      toastNotifier.push({
        level: "info",
        title: "Fixture Info!",
        msg: "Fixture types updated!",
      });
    }
    if (data == "ProjectSaved") {
      toastNotifier.push({
        level: "info",
        title: "Project Info!",
        msg: "Project saved succsessfully!",
      });
    }
  });
  // https://svelte.dev/repl/8b974ea483c648fba362a1e9f3dbc29f?version=4.2.8
  // https://svelte.dev/repl/fd9d2216e7e243d49de8fae39ecc6fe8?version=3.37.0
  // https://svelte.dev/repl/8c2d03bcc8434a158f01a80fc624c636?version=4.2.2
  // https://svelte-grid.vercel.app/features
  let pane: "configure" | "program" | "show" = "configure";
</script>

<main>
  <Headbar bind:pane></Headbar>
  <DisconnectHelper></DisconnectHelper>
  <Toast></Toast>
  <!-- {$info} -->

  <div class="panes">
    {#if pane === "configure"}
      <ConfigurePanel></ConfigurePanel>
    {:else if pane === "program"}
      <ProgramPanel></ProgramPanel>
    {:else if pane === "show"}
      <h1>Show</h1>
    {/if}
  </div>

  <!-- <div>
    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
      <img src={viteLogo} class="logo" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank" rel="noreferrer">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>
  <h1>Vite + Svelte</h1>

  <div class="card">
    <Counter />
  </div>

  <p>
    Check out <a href="https://github.com/sveltejs/kit#readme" target="_blank" rel="noreferrer">SvelteKit</a>, the official Svelte app framework powered by Vite!
  </p>

  <p class="read-the-docs">
    Click on the Vite and Svelte logos to learn more
  </p> -->
  <!-- {#await promise}
    <p>Still Loading</p>
  {:then data}
    <code>{JSON.stringify(data, undefined, 4)}</code>
  {/await} -->
</main>

<!-- <style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
  .read-the-docs {
    color: #888;
  }
</!-->

<style>
  .panes {
    width: 100%;
    height: calc(100vh - 3rem);
    display: grid;
    background-color: transparent;
    align-items: center;
    justify-items: center;
  }
</style>

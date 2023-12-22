<script lang="ts">
  import DisconnectHelper from "./lib/DisconnectHelper.svelte";
  import Headbar from "./lib/Headbar.svelte";
  import Toast from "./lib/Toast.svelte";
  import UniverseExplorer from "./lib/UniverseExplorer.svelte";
  import { info, toastNotifier } from "./lib/stores";

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

  let pane: "configure" | "program" | "show" = "configure";
</script>

<main>
  <Headbar bind:pane></Headbar>
  <DisconnectHelper></DisconnectHelper>
  <Toast></Toast>
  {$info}
  <UniverseExplorer></UniverseExplorer>

  {#if pane === "configure"}
    <h1>Configure</h1>
    <button
      on:click={() =>
        toastNotifier.push({
          level: "info",
          title: "Info!",
          msg: "This is an info prompt.",
        })}>Info</button
    >
    <button
      on:click={() =>
        toastNotifier.push({
          level: "warning",
          title: "Warning!",
          msg: "This is an warning prompt.",
        })}>Warning</button
    >
    <button
      on:click={() =>
        toastNotifier.push({
          level: "error",
          title: "Error!",
          msg: "An error occured.",
        })}>Error</button
    >
  {:else if pane === "program"}
    <h1>Program</h1>
  {:else if pane === "show"}
    <h1>Show</h1>
  {/if}

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
  {#await promise}
    <p>Still Loading</p>
  {:then data}
    <code>{JSON.stringify(data, undefined, 4)}</code>
  {/await}
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
</style> -->

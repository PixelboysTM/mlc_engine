<script lang="ts">
  import DisconnectHelper from "./lib/DisconnectHelper.svelte";
  import FaFileUpload from "svelte-icons/fa/FaFileUpload.svelte";
  import FaPlus from "svelte-icons/fa/FaPlus.svelte";
  // import Headbar from "./lib/Headbar.svelte";
  // import Toast from "./lib/Toast.svelte";
  // import UniverseExplorer from "./lib/UniverseExplorer.svelte";
  // import { info, toastNotifier } from "./lib/stores";

  // let promise = getFixtureTypes();

  // async function getFixtureTypes() {
  //   const response = await fetch("/data/get/fixture-types");
  //   const data = await response.json();
  //   return data;
  // }

  // let s = info.subscribe((data) => {
  //   console.log(data);
  //   if (data == "FixtureTypesUpdated") {
  //     promise = getFixtureTypes();
  //     toastNotifier.push({
  //       level: "info",
  //       title: "Fixture Info!",
  //       msg: "Fixture types updated!",
  //     });
  //   }
  //   if (data == "ProjectSaved") {
  //     toastNotifier.push({
  //       level: "info",
  //       title: "Project Info!",
  //       msg: "Project saved succsessfully!",
  //     });
  //   }
  // });
  // // https://svelte.dev/repl/8b974ea483c648fba362a1e9f3dbc29f?version=4.2.8
  // // https://svelte.dev/repl/fd9d2216e7e243d49de8fae39ecc6fe8?version=3.37.0
  // // https://svelte.dev/repl/8c2d03bcc8434a158f01a80fc624c636?version=4.2.2
  // // https://svelte-grid.vercel.app/features
  // let pane: "configure" | "program" | "show" = "configure";
</script>

<svelte:head>
  <title>MLC Project Browser</title>
  <link rel="icon" href="/favicon.ico" />
</svelte:head>
<main>
  <div>
    <span
      ><a id="a" href="#top">M</a><a id="b" href="#top">L</a><a
        id="c"
        href="#top">C</a
      ></span
    >
    <div class="tabs">
      <h2>Project Browser</h2>
    </div>
    <div class="tabs right">
      <button title="Import Project" class="icon"><FaFileUpload /></button>
      <button
        title="Create New Project"
        class="icon"
        on:click={() => fetch("/data/save")}><FaPlus /></button
      >
    </div>
  </div>

  <!-- <DisconnectHelper></DisconnectHelper>
  <Toast></Toast> -->
  <!-- {$info} -->

  <!-- <div class="panes">
    {#if pane === "configure"}
      <div class="configure">
        <UniverseExplorer></UniverseExplorer>
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
      </div>
    {:else if pane === "program"}
      <h1>Program</h1>
    {:else if pane === "show"}
      <h1>Show</h1>
    {/if}
  </div> -->

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

<!-- <style>
  .panes {
    width: 100%;
    height: calc(100vh - 3rem);
    display: grid;
    background-color: transparent;
    align-items: center;
    justify-items: center;
  }
</style> -->

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

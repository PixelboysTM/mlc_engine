<script lang="ts">
  import svelteLogo from "../assets/svelte.svg";
  import UploadFixture from "./UploadFixture.svelte";
  import viteLogo from "/vite.svg";

  let showUpload = false;
</script>

<div>
  <span
    ><a id="a" href="/">M</a><a id="b" href="/">L</a><a id="c" href="/">C</a
    ></span
  >
  <div class="tabs">
    <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
      <img src={viteLogo} class="logo" alt="Vite Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank" rel="noreferrer">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>
  <button on:click={() => (showUpload = true)}>Upload</button>

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
</style>

<script lang="ts">
  import DisconnectHelper from "./lib/DisconnectHelper.svelte";
  import FaFileUpload from "svelte-icons/fa/FaFileUpload.svelte";
  import FaPlus from "svelte-icons/fa/FaPlus.svelte";
  import MdOpenInBrowser from "svelte-icons/md/MdOpenInBrowser.svelte";
  import Grid from "svelte-grid";
  import gridHelp from "svelte-grid/build/helper/index.mjs";

  type ProjectInformation = {
    name: string;
    lastSaved: Date;
  };

  let items = [
    make_item(0),
    make_item(1),
    // make_item(2),
    // make_item(3),
    // make_item(4),
    // make_item(5),
    // make_item(6),
    // make_item(7),
    // make_item(8),
    // make_item(9),
    // make_item(10),
    // make_item(11),
    // make_item(12),
    // make_item(13),
    // make_item(14),
  ];

  function make_item(i: number) {
    return {
      5: gridHelp.item({
        x: i % 5,
        y: Math.floor(i / 5) * 3,
        w: 1,
        h: 3,
        resizable: false,
        draggable: false,
      }),
      id: i,
      data: {
        name: "Project " + i,
        lastSaved: new Date(),
      } as ProjectInformation,
    };
  }
  const breakpoint = 1200;
  const column = 5;

  const cols = [[breakpoint, column]];
</script>

<svelte:head>
  <title>MLC Project Browser</title>
  <link rel="icon" href="/favicon.ico" />
</svelte:head>
<main>
  <DisconnectHelper></DisconnectHelper>
  <div class="head">
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
  <div class="grid">
    <Grid {cols} bind:items let:item let:dataItem rowHeight={100}>
      <div class="project-card">
        <h4>{dataItem.data.name}</h4>
        <p>{dataItem.data.lastSaved.toLocaleString()}</p>
        <button class="icon">Open</button>
      </div>
    </Grid>
  </div>
</main>

<style>
  div.project-card {
    background-color: #151111;
    border-radius: 0.5rem;
    /* margin: 1rem; */
    color: #fff;
    font-family: "Roboto Mono", monospace;
    font-weight: 700;
    font-size: 1.5rem;
    display: flex;
    flex-direction: column;
    place-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
  }
  .project-card h4 {
    margin: 0rem;
    padding: 0rem;
    margin-top: 1rem;
  }
  .project-card p {
    margin: 0rem;
    padding: 0rem;
    font-size: medium;
    margin-bottom: auto;
  }
  .project-card button {
    margin-bottom: 1rem;
  }
  div.grid {
    /* width: 100%; */
    margin: 0rem;
    margin-left: auto;
    margin-right: auto;
    height: calc(100vh - 3rem);
    background-color: transparent;
    display: block;
    /* overflow: hidden; */
    padding: 0rem;
    margin-top: 3rem;
  }

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
  .head {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1000;
    border-bottom: #151311 1px solid;
    animation: border 10s linear 0s infinite;
  }

  @keyframes border {
    0% {
      border-bottom: #ff4e3e 1px solid;
    }
    33% {
      border-bottom: #3eff41 1px solid;
    }
    66% {
      border-bottom: #3e88ff 1px solid;
    }
    100% {
      border-bottom: #ff4e3e 1px solid;
    }
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
</style>

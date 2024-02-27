<script lang="ts">
  import DisconnectHelper from "./lib/DisconnectHelper.svelte";
  import { FileUp, Plus, FolderOpen } from "lucide-svelte";
  import Grid from "svelte-grid";
  import gridHelp from "svelte-grid/build/helper/index.mjs";

  import marvin from "./assets/icon.png";
  import { toastNotifier } from "./lib/stores";
  import Toast from "./lib/Toast.svelte";

  type ProjectInformation = {
    name: string;
    file_name: string;
    last_edited: string;
  };

  fetch("/projects/projects-list").then((res) => {
    res.json().then((data) => {
      let i = 0;
      items = data.map((item: ProjectInformation) => {
        return make_item(i++, item);
      });
    });
  });

  let items: any[] = [];

  function make_item(i: number, data: ProjectInformation) {
    return {
      5: gridHelp.item({
        x: i % 5,
        y: Math.floor(i / 5) * 3,
        w: 1,
        h: 3,
        resizable: false,
        draggable: false,
      }),
      4: gridHelp.item({
        x: i % 4,
        y: Math.floor(i / 4) * 3,
        w: 1,
        h: 3,
        resizable: false,
        draggable: false,
      }),
      3: gridHelp.item({
        x: i % 3,
        y: Math.floor(i / 3) * 3,
        w: 1,
        h: 3,
        resizable: false,
        draggable: false,
      }),
      2: gridHelp.item({
        x: i % 2,
        y: Math.floor(i / 2) * 3,
        w: 1,
        h: 3,
        resizable: false,
        draggable: false,
      }),
      1: gridHelp.item({
        x: i % 1,
        y: Math.floor(i / 1) * 3,
        w: 1,
        h: 3,
        resizable: false,
        draggable: false,
      }),
      id: i,
      data: data,
    };
  }

  function format_time(timeString: string) {
    let time = new Date(timeString);
    return (
      time.getDate() +
      "." +
      (time.getMonth() + 1) +
      "." +
      time.getFullYear() +
      " " +
      time.getHours() +
      ":" +
      time.getMinutes()
    );
  }

  const cols = [
    [1500, 5],
    [1200, 4],
    [950, 3],
    [600, 2],
    [400, 1],
    [0, 0],
  ];
</script>

<svelte:head>
  <title>MLC Project Browser</title>
  <meta name="viewport" content="width=device-width,initial-scale=1.0" />
  <!-- Add a Favicon -->
  <link rel="icon" href={marvin} />
</svelte:head>
<main>
  <DisconnectHelper></DisconnectHelper>
  <Toast></Toast>
  <div class="head">
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <img
      on:click={() => {
        toastNotifier.push({
          level: "info",
          msg: "Marvin says hi!",
          title: "Marvin",
        });
      }}
      on:keypress
      class="iconMarvin"
      src={marvin}
      alt="MLC"
    />
    <div class="tabs">
      <h2>MLC Project Browser</h2>
    </div>
    <div class="tabs right">
      <button title="Import Project" class="icon"
        ><FileUp size={"1rem"} /></button
      >
      <button title="Create New Project" class="icon"
        ><Plus size={"1rem"} /></button
      >
    </div>
  </div>
  <div class="grid">
    <Grid {cols} bind:items let:item let:dataItem rowHeight={100}>
      <div class="project-card">
        <h4>{dataItem.data.name}</h4>
        <p>
          {dataItem.data.file_name}
        </p>
        <p>
          {format_time(dataItem.data.last_edited)}
        </p>
        <button
          on:click={() => {
            fetch("/projects/load/" + dataItem.data.file_name).then((data) =>
              window.location.reload()
            );
          }}
          ><FolderOpen size={"1rem"} />
          <p class="loadBtn">Open</p></button
        >
      </div>
    </Grid>
  </div>
</main>

<style>
  .iconMarvin {
    width: 3rem;
    height: 80%;
    margin-left: 0.2rem;
    margin-top: 0.25rem;
    cursor: pointer;
  }
  div.project-card {
    background-color: var(--color-panel);
    border-radius: var(--number-border-radius);
    /* margin: 1rem; */
    color: var(--color-text);
    font-family: "Roboto Mono", monospace;
    font-weight: 700;
    font-size: 1.5rem;
    display: flex;
    flex-direction: column;
    place-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    touch-action: auto;
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
  .head {
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1000;
    animation: border 10s linear 0s infinite;
  }

  button p.loadBtn {
    display: none;
    transition: all 0.75s ease-in-out;
  }

  button {
    transition: width 0.75s ease-in-out;
  }

  button:hover p.loadBtn {
    display: block;
  }

  @keyframes border {
    0% {
      border-bottom: var(--color-accent) 1px solid;
    }
    33% {
      border-bottom: var(--color-secondary) 1px solid;
    }
    66% {
      border-bottom: var(--color-tertiary) 1px solid;
    }
    100% {
      border-bottom: var(--color-accent) 1px solid;
    }
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

  button {
    padding: 1rem;
    align-items: center;
    display: flex;
  }

  /* Extra small devices (phones, 600px and down) */
  @media only screen and (max-width: 600px) {
    .tabs h2 {
      font-size: 1rem;
    }
  }
</style>

<script lang="ts">
  import DisconnectHelper from "./lib/DisconnectHelper.svelte";
  // import FaFileUpload from "svelte-icons/fa/FaFileUpload.svelte";
  // import FaPlus from "svelte-icons/fa/FaPlus.svelte";
  import FaSolidFileUpload from "svelte-icons-pack/fa/FaSolidFileUpload";
  import FaSolidPlus from "svelte-icons-pack/fa/FaSolidPlus";
  // import MdOpenInBrowser from "svelte-icons/md/MdOpenInBrowser.svelte";
  import FaSolidFolderOpen from "svelte-icons-pack/fa/FaSolidFolderOpen";
  import Icon from "svelte-icons-pack/Icon.svelte";
  import Grid from "svelte-grid";
  import gridHelp from "svelte-grid/build/helper/index.mjs";

  type ProjectInformation = {
    name: string;
    file_name: string;
    last_edited: string;
  };

  fetch("/projects/projects-list").then((res) => {
    res.json().then((data) => {
      console.log(data);
      let i = 0;
      items = data.map((item: ProjectInformation) => {
        return make_item(i++, item);
      });
    });
  });

  let items: any[] = [
    // make_item(0),
    // make_item(1),
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
    console.log(timeString);
    let time = new Date(timeString);
    console.log(time);

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
  // const breakpoint = 0;
  // const column = 5;

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
      <h2>Project Browser 2</h2>
    </div>
    <div class="tabs right">
      <button title="Import Project" class="icon"
        ><Icon color="white" src={FaSolidFileUpload} /></button
      >
      <button title="Create New Project" class="icon"
        ><Icon color="white" src={FaSolidPlus} /></button
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
          ><Icon color="white" src={FaSolidFolderOpen} />
          <p class="loadBtn">Open</p></button
        >
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

  /* Small devices (portrait tablets and large phones, 600px and up) */
  @media only screen and (min-width: 600px) {
    .example {
      background: green;
    }
  }

  /* Medium devices (landscape tablets, 768px and up) */
  @media only screen and (min-width: 768px) {
    .example {
      background: blue;
    }
  }

  /* Large devices (laptops/desktops, 992px and up) */
  @media only screen and (min-width: 992px) {
    .example {
      background: orange;
    }
  }

  /* Extra large devices (large laptops and desktops, 1200px and up) */
  @media only screen and (min-width: 1200px) {
    .example {
      background: pink;
    }
  }
</style>

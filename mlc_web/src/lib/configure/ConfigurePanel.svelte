<script lang="ts">
  import App from "../../App.svelte";
  import FixtureTypeExplorer from "./FixtureTypeExplorer.svelte";
  import DualRingSpinner from "../misc/DualRingSpinner.svelte";
  import FaderPanel from "../common/FaderPanel.svelte";
  import UniverseExplorer from "./UniverseExplorer.svelte";
  import Project from "../../Project.svelte";
  import ProjectSettings from "./ProjectSettings.svelte";

  let projectInfo:
    | undefined
    | { name: string; last_edited: string; file_name: string };
  fetch("/projects/current")
    .then((response) => response.json())
    .then((data) => {
      projectInfo = data;
    });

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
</script>

<div class="configure">
  <div class="info panel">
    <h3>Project Info:</h3>
    {#if projectInfo != undefined}
      <p><span class="pis">Name:</span> {projectInfo.name}</p>
      <p><span class="pis">Filename:</span> {projectInfo.file_name}</p>
      <p>
        <span class="pis">Last saved:</span>{format_time(
          projectInfo.last_edited
        )}
      </p>
    {:else}
      <div class="center">
        <DualRingSpinner></DualRingSpinner>
      </div>
    {/if}
  </div>
  <div class="fixture-types panel">
    <h3>Fixture Types:</h3>
    <FixtureTypeExplorer></FixtureTypeExplorer>
  </div>
  <div class="universe-explorer panel">
    <h3>Universe Explorer:</h3>
    <UniverseExplorer></UniverseExplorer>
  </div>
  <div class="project-settings panel">
    <h3>Project Settings:</h3>
    <ProjectSettings></ProjectSettings>
  </div>
  <div class="faders panel">
    <h3>Faders:</h3>
    <FaderPanel></FaderPanel>
  </div>
</div>

<!-- 

<FixtureTypeExplorer></FixtureTypeExplorer> -->

<style>
  .configure {
    width: calc(100% - 1rem);
    height: calc(100% - 1rem);
    display: grid;
    padding: 0.5rem;

    grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
    grid-template-rows: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
    gap: 0.5rem;

    min-height: 0;
    min-width: 0;
  }

  .panel {
    width: calc(100% - 2rem);
    /*height: calc(100% - 2rem);*/
    padding: 1rem;
    padding-top: 0rem;
    padding-bottom: 0rem;
    border-radius: 0.25rem;
    background-color: #151111;
    overflow: auto;
    min-width: 0;
  }

  .info {
    grid-column: 1 / 4;
    grid-row: 1 / 4;
    align-items: center;
    justify-content: center;
  }
  .fixture-types {
    grid-column: 1 / 4;
    grid-row: 4 / 13;
  }
  .universe-explorer {
    grid-column: 4 / 11;
    grid-row: 1 / 9;
  }
  .project-settings {
    grid-column: 11 / 13;
    grid-row: 1 / 9;
  }
  .faders {
    grid-column: 4 / 13;
    grid-row: 9 / 13;
  }
  span.pis {
    width: 9ch;
    display: inline-block;
    text-align: end;
    margin-right: 2ch;
  }
  .panel h3 {
    text-align: center;
    margin: 0;
    width: 100%;
    /* margin-bottom: 0.25rem; */
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    position: sticky;
    top: 0px;
    background-color: #151111;
    padding-top: 0.25rem;
    z-index: 15;
  }

  .center {
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>

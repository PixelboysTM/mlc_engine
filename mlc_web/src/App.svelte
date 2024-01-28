<script lang="ts">
  import DisconnectHelper from "./lib/DisconnectHelper.svelte";
  import Headbar from "./lib/Headbar.svelte";
  import Toast from "./lib/Toast.svelte";
  import ConfigurePanel from "./lib/configure/ConfigurePanel.svelte";
  import { info, toastNotifier } from "./lib/stores";
  import ProgramPanel from "./lib/program/ProgramPanel.svelte";
  import marvin from "./assets/icon.png";

  let s = info.subscribe((data) => {
    console.log(data);
    if (data == "FixtureTypesUpdated") {
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

<svelte:head>
  <title>Marvin Light Controller</title>
  <meta name="viewport" content="width=device-width,initial-scale=1.0" />
  <link rel="icon" href={marvin} />
</svelte:head>
<main>
  <Headbar bind:pane></Headbar>
  <DisconnectHelper></DisconnectHelper>
  <Toast></Toast>

  <div class="panes">
    {#if pane === "configure"}
      <ConfigurePanel></ConfigurePanel>
    {:else if pane === "program"}
      <ProgramPanel></ProgramPanel>
    {:else if pane === "show"}
      <h1>Show</h1>
    {/if}
  </div>
</main>

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

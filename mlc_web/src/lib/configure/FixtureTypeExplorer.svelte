<script lang="ts">
  import DualRingSpinner from "../misc/DualRingSpinner.svelte";
  import { info } from "../stores";
  import FixtureType from "./FixtureType.svelte";

  let fixtureTypes: Promise<{ name: string; id: string; modes: string[] }[]>;

  loadTypes();

  let s = info.subscribe((data) => {
    console.log(data);
    if (data == "FixtureTypesUpdated") {
      loadTypes();
    }
  });

  function loadTypes() {
    fixtureTypes = fetch("/data/get/fixture-types")
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        return data;
      });
  }
</script>

<div class="fixtures">
  {#await fixtureTypes}
    <div class="center">
      <DualRingSpinner></DualRingSpinner>
    </div>
  {:then ts}
    {#each ts as t}
      <FixtureType fixtureType={t}></FixtureType>
    {/each}
  {:catch error}
    <div class="error">{error.message}</div>
  {/await}
</div>

<style>
  .center {
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>

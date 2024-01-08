<script lang="ts">
  import { info } from "../stores";

  let fixtureTypes: Promise<{ name: string; id: string; modes: string[] }> =
    fetch("/data/get/fixture-types")
      .then((res) => res.json())
      .then((data) => {
        console.log(data);
        return data;
      });
</script>

<div class="fixtures">
  {#await fixtureTypes}
    <div class="loading">Loading...</div>
  {:then types}
    {types}
  {/await}
</div>

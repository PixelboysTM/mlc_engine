<script lang="ts">
  import FixtureTypeExplorer from "./FixtureTypeExplorer.svelte";

  let projectInfo:
    | undefined
    | { name: string; last_edited: string; file_name: string };
  fetch("/projects/current")
    .then((response) => response.json())
    .then((data) => {
      projectInfo = data;
    });
</script>

{#if projectInfo != undefined}
  <h1>{projectInfo.name}</h1>
  <p>{projectInfo.file_name}</p>
  <p>{projectInfo.last_edited}</p>
{/if}

<FixtureTypeExplorer></FixtureTypeExplorer>

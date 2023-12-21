<script lang="ts">
  import { info } from "./stores";
  let promise: any;
  getUniverses();

  function getUniverses() {
    fetch("/data/universes")
      .then((res) => res.json())
      .then((res) => {
        promise = res;
      })
      .catch((err) => {
        console.log(err);
      });
  }

  info.subscribe((data) => {
    if (data == "UniversesUpdated") {
      getUniverses();
    }
  });
</script>

{#await promise then universes}
  <code>{universes}</code>
{:catch error}
  <p>Error loading universes</p>
{/await}

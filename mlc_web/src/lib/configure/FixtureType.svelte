<script lang="ts">
  export let fixtureType: { name: string; id: string; modes: string[] };
</script>

<div class="fixture-type">
  <h3>{fixtureType.name}</h3>
  <p>{fixtureType.id}</p>
  <p>{fixtureType.modes}</p>
</div>

<style>
  h3 {
    margin: 0;
    width: 100%;
    background-color: #1a1a1a;
    padding: 0.1rem 0.25rem;
    border-radius: 0.1rem;
  }

  p {
    margin: 0 0.2rem;
    width: 100%;
  }
  .fixture-type {
    margin-bottom: 1rem;
  }
</style>

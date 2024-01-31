<script lang="ts">
  import { toastNotifier } from "../stores";
  export let fixtureType: { name: string; id: string; modes: string[] };

  function patch(e: MouseEvent, mode: number) {
    if (!e.ctrlKey) {
      return;
    }
    fetch(
      "/data/patch/" + fixtureType.id + "/" + mode + (e.altKey ? "?create" : "")
    )
      .then((r) => {
        if (r.status == 200) {
          toastNotifier.push({
            title: "Patch",
            level: "info",
            msg: "Patched succsessfully!",
          });
        }
        if (r.status == 400) {
          toastNotifier.push({
            title: "Patch",
            level: "error",
            msg: "The Patch request was wrong: " + r.body,
          });
        }
        if (r.status == 409) {
          toastNotifier.push({
            title: "Patch",
            level: "warning",
            msg: "The Fixture can not fit in the Universe! (Use Strg+Alt+Click to create a new Universe)",
          });
        }
      })
      .catch((err) => {
        console.log(err);
        toastNotifier.push({
          title: "Patch",
          level: "error",
          msg: "Patched failed: " + err.message,
        });
      });
  }
</script>

<div class="fixture-type">
  <h3>{fixtureType.name}</h3>
  <p class="f-id">{fixtureType.id}</p>
  {#each fixtureType.modes as mode, i}
    <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <p
      data-tooltip="Try patch this Mode ta a Universe (Strg+Click)"
      class="p-btn"
      on:click={(e) => patch(e, i)}
      on:keypress
      tabindex={0}
    >
      ⏺ {mode}
    </p>
  {/each}
</div>

<style>
  [data-tooltip] {
    position: relative;
    cursor: help;
  }

  [data-tooltip]::after {
    position: absolute;
    opacity: 0;
    pointer-events: none;
    content: attr(data-tooltip);
    left: 0;
    top: calc(100% + 10px);
    border-radius: 3px;
    /* box-shadow: 0 0 5px 2px rgba(100, 100, 100, 0.6); */
    background-color: var(--color-panel);
    z-index: 10;
    padding: 8px;
    width: 15rem;
    color: var(--color-text);
    /* transform: translateY(-20px);
    transition: all 150ms cubic-bezier(0.25, 0.8, 0.25, 1); */
  }

  [data-tooltip]:hover::after {
    opacity: 1;
    transform: translateY(0);
    /* transition-duration: 300ms; */
  }
  h3 {
    margin: 0;
    width: 100%;
    background-color: var(--color-background);
    padding: 0.1rem 0.25rem;
    border-radius: 0.1rem;
  }

  p {
    margin: 0 0.2rem;
    width: 100%;
  }

  .p-btn {
    cursor: pointer;
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
  }
  .p-btn:hover {
    color: var(--color-accent);
  }

  .f-id {
    font-size: 0.75rem;
    color: #888;
  }
  .fixture-type {
    margin-bottom: 1rem;
  }
</style>

<script lang="ts">
  import DualRingSpinner from "./misc/DualRingSpinner.svelte";
  import { info, type InfoKind } from "./stores";

  let active = false;
  info.subscribe((data) => {
    handleData(data);
  });

  function handleData(data: InfoKind) {
    if (data == "SystemShutdown") {
      setActive();
    }
  }

  function setActive() {
    active = true;
    clearInterval(inter);
  }

  let failed = 0;
  function request_heartbeat() {
    fetch("/util/heartbeat")
      .then((res) => res.json())
      .then((data) => {
        // console.log(data);
        failed = 0;
      })
      .catch((err) => {
        console.log(err);
        failed++;
        if (failed > 5) {
          setActive();
        } else {
          request_heartbeat();
        }
      });
  }

  let inter = setInterval(() => {
    request_heartbeat();
  }, 5000);
</script>

{#if active}
  <div class="bg">
    <div class="inner">
      <h3>Backend shutdown please restart and reload!</h3>
      <div class="center">
        <DualRingSpinner></DualRingSpinner>
      </div>
      <button on:click={() => window.location.reload()}>Reload</button>
    </div>
  </div>
{/if}

<style>
  div.bg {
    background-color: var(--color-background-transparent);
    width: 100%;
    height: 100%;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 2000;
  }
  div.inner {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: var(--color-text);
    background-color: var(--color-panel);
    border-radius: var(--number-border-radius);
    padding: 2rem;
    display: flex;
    flex-direction: column;
  }
  .center {
    display: flex;
    justify-content: center;
    align-items: center;
    margin: 1rem;
  }
  h3 {
    margin: 0;
    padding: 0;
  }
</style>

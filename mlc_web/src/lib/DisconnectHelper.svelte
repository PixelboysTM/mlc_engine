<script lang="ts">
  import { info } from "./stores";

  let active = false;
  info.subscribe((data) => {
    handleData(data);
  });

  function handleData(data: string) {
    if (data == "SystemShutdown") {
      active = true;
    }
  }
</script>

{#if active}
  <div class="bg">
    <div class="inner">
      <h3>Backend shutdown please restart and reload!</h3>
    </div>
  </div>
{/if}

<style>
  div.bg {
    background-color: #00000061;
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
    color: #ffffff;
    background-color: #4a4a4a;
    border-radius: 2rem;
    padding: 2rem;
    display: flex;
    flex-direction: column;
  }
  textarea {
    resize: none;
  }
</style>

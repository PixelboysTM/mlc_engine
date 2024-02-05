<script>
  import {
    Plus,
    Play,
    Rewind,
    FastForward,
    ZoomIn,
    ZoomOut,
  } from "lucide-svelte";
</script>

<div class="timeline">
  <div class="toolbar">
    <div class="left">
      <div class="iconBtn"><Plus size="1rem" /></div>
    </div>
    <div class="middle">
      <div class="iconBtn"><Rewind size="1rem" /></div>
      <div class="iconBtn"><Play size="1rem" /></div>
      <div class="iconBtn"><FastForward size="1rem" /></div>
    </div>
    <div class="right">
      <div class="iconBtn"><ZoomOut size="1rem" /></div>
      <div class="iconBtn"><ZoomIn size="1rem" /></div>
    </div>
  </div>
  <div class="tracks">Trackss</div>
</div>

<style>
  .timeline {
    width: 100%;
    height: 100%;
  }

  .toolbar {
    width: 100%;
    height: 2.5rem;
    /* background-color: #333; */
    border-bottom: 1px solid var(--color-accent);
    display: grid;
    grid-template-columns: auto auto auto;
  }

  .toolbar .left {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    gap: 0.25rem;
  }

  .toolbar .middle {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 0.25rem;
  }

  .toolbar .right {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: 0.25rem;
  }

  .iconBtn {
    width: 1.5rem;
    height: 1.5rem;
    display: flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;
    background-color: var(--color-button);
    border-radius: var(--number-border-radius);
  }
</style>

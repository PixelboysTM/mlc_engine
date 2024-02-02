<div class="outer">
  <div class="lds-dual-ring" />
</div>

<style>
  .outer {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
  }
  .lds-dual-ring {
    display: flex;
    align-content: center;
    align-items: center;
    width: 80px;
    height: 80px;
    border-bottom: var(--color-accent) 6px solid;
    border-left: var(--color-secondary) 6px solid;
    border-top: var(--color-tertiary) 6px solid;
    border-right: var(--color-text) 6px solid;
    border-radius: 50%;
    animation: spin-animation 2s linear infinite;
  }
  @keyframes spin-animation {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>

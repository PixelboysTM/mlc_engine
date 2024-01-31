<script>
  import marvin from "../../assets/icon.png";
</script>

<div class="outer">
  <div class="lds-dual-ring">
    <!-- <img class="iconMarvin" src={marvin} alt="MLC" /> -->
  </div>
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
    animation: lds-dual-ring 2s linear infinite;
  }
  /* .lds-dual-ring:after {
    content: " ";
    display: block;
    width: 64px;
    height: 64px;
    margin: 8px;
    border-radius: 50%;
    border: 6px solid #fff;
    border-color: #fff transparent #fff transparent;
    animation: lds-dual-ring 1.2s linear infinite;
  } */
  @keyframes lds-dual-ring {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>

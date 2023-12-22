<script lang="ts">
  import { toastNotifier, type ToastNotification } from "./stores";

  let level: "info" | "warning" | "error" = "info";
  let title: string = "ToastManager";
  let msg: string = "Notificationsmanager loaded";

  let interval: number | undefined = undefined;
  toastNotifier.subscribe((data) => {
    if (interval != undefined) {
      return;
    }

    handleData();
    interval = setInterval(() => {
      handleData();
    }, 5000);
  });

  function handleData() {
    let data = toastNotifier.pull();
    if (data == null) {
      clearInterval(interval);
      interval = undefined;
      return;
    }

    let d: ToastNotification = data;

    level = d.level;
    title = d.title;
    msg = d.msg;
  }
</script>

{#if interval != undefined}
  <div class="toast {level}">
    <h4>{title}</h4>
    <p>{msg}</p>
  </div>
{/if}

<style>
  div.toast {
    position: fixed;
    bottom: 0.5rem;
    right: 0.5rem;
    width: 12rem;
    height: 3rem;
    background-color: #151111;
    color: white;
    z-index: 1001;
    padding: 0.5rem;
    border-radius: 1rem;
    box-shadow: -0.2rem 0.2rem 1rem #2d2d2db3;
    animation: slideIn 5s linear 0s infinite;
  }

  @keyframes slideIn {
    0% {
      opacity: 0%;
    }
    2.5% {
      opacity: 0%;
    }
    7.5% {
      opacity: 100%;
    }
    92.5% {
      opacity: 100%;
    }
    97.5% {
      opacity: 0%;
    }
    100% {
      opacity: 0%;
    }
  }

  h4 {
    margin: 0;
    padding: 0;
    font-size: 1rem;
  }

  p {
    margin: 0;
    padding: 0;
    font-size: 0.8rem;
  }

  div.toast.info {
    background-color: #151111;
  }
  div.toast.warning {
    background-color: #ffbf3e;
  }
  div.toast.error {
    background-color: #ff3e3e;
  }
</style>

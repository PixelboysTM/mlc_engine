<script lang="ts">
  import { toastNotifier, type ToastNotification } from "./stores";
  import { createToaster, melt } from "@melt-ui/svelte";
  import { flip } from "svelte/animate";
  import { fly } from "svelte/transition";

  // let level: "info" | "warning" | "error" = "info";
  // let title: string = "ToastManager";
  // let msg: string = "Notificationsmanager loaded";

  // let interval: number | undefined = undefined;
  toastNotifier.subscribe((data) => {
    // if (interval != undefined) {
    //   return;
    // }

    // handleData();
    // interval = setInterval(() => {
    //   handleData();
    // }, 2000);
    // handleData();
    if (data == null) {
      return;
    }
    addToast({
      data: data as ToastNotification,
      // closeDelay: 1000,
    });
  });

  const {
    elements: { content, title, description },
    helpers: { addToast },
    states: { toasts },
    actions: { portal },
  } = createToaster<ToastNotification>();
</script>

<div class="portal" use:portal>
  {#each $toasts as { id, data } (id)}
    <div
      class="toast t-{data.level}"
      use:melt={$content(id)}
      animate:flip={{ duration: 500 }}
      in:fly={{ x: "100%", duration: 150 }}
      out:fly={{ x: "100%", duration: 150 }}
    >
      <h4 use:melt={$title(id)}>
        {data.title}
        <span class="dot {data.level}"></span>
      </h4>
      <p use:melt={$description(id)}>{data.msg}</p>
    </div>
  {/each}
</div>

<style>
  div.toast {
    width: 12rem;
    height: 3rem;
    background-color: var(--color-background);
    color: var(--color-text);
    padding: 0.5rem;
    border-radius: var(--number-border-radius);
    /* box-shadow: -0.2rem 0.2rem 1rem #2d2d2db3; */
    /* border: #ff3e3e 1px solid; */
  }

  .dot {
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 50%;
    display: inline-block;
    margin-left: 0.5rem;
  }

  .portal {
    position: fixed;
    bottom: 0.25rem;
    right: 0.25rem;
    z-index: 50;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    place-items: flex-end;
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

  .info {
    background-color: var(--color-info);
  }
  .warning {
    background-color: var(--color-warning);
  }
  .error {
    background-color: var(--color-error);
  }

  .t-info {
    border: var(--color-info) 1px solid;
  }
  .t-warning {
    border: var(--color-warning) 1px solid;
  }
  .t-error {
    border: var(--color-error) 1px solid;
  }
</style>

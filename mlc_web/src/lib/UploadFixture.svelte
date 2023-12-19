<script lang="ts">
  import { createEventDispatcher } from "svelte";

  let inputJson: HTMLTextAreaElement;

  let errorMsg = "";

  const dispatch = createEventDispatcher();

  function close() {
    dispatch("close");
  }

  function upload() {
    const json = inputJson.value;
    fetch("/data/add/fixture", {
      method: "POST",
      body: json,
    })
      .then((res) => {
        if (res.ok) {
          errorMsg = '<span style="color:green;">Upload successful</span>';
          inputJson.value = "";
        } else {
          console.log(res);
          errorMsg = `<span style="color:red;">Upload failed. See console for error.</span>`;
        }
      })
      .catch((err) => {
        console.log(err);
        errorMsg =
          '<span style="color:red;">Upload failed. See console for error.</span>';
      });
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="bg">
  <div class="inner">
    <div>Paste OFL JSON and click Upload</div>
    <textarea bind:this={inputJson} rows="40" cols="100" />
    <div class="btns">
      <button on:click={upload}>Upload</button>
      <button on:click={close}>Close</button>
    </div>
    <span>{@html errorMsg}</span>
  </div>
</div>

<style>
  div.bg {
    background-color: #00000061;
    width: 100%;
    height: 100%;
    position: fixed;
    top: 0;
    left: 0;
    z-index: 1000;
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

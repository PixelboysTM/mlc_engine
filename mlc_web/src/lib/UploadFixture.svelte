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
    <h3>Upload Fixture Definition</h3>
    <p>Paste AGLight JSON and click Upload</p>
    <textarea bind:this={inputJson} rows="35" cols="100" />
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
  h3 {
    margin: 0;
  }
  p {
    margin: 0.5rem 0;
  }
  div.inner {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: #ffffff;
    background-color: #4a4a4a;
    border-radius: 2rem;
    padding: 1.5rem 2rem;
    display: flex;
    flex-direction: column;
  }
  textarea {
    resize: none;
  }

  div.btns {
    display: flex;
    justify-content: flex-start;

    padding-top: 0.25rem;
  }
  div.btns button {
    margin-right: 0.5rem;
  }
</style>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  let inputJson: HTMLTextAreaElement;

  let errorMsg = "";

  const dispatch = createEventDispatcher();

  function close() {
    dispatch("close");
  }

  function upload(json: string) {
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

  let source: "OFL" | "JSON" = "OFL";
  let search: string = "";
  let availableFixtures: { manufacturer: string; name: string }[] = [];
  fetch("https://open-fixture-library.org/api/v1/get-search-results", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    mode: "cors",
    body: JSON.stringify({
      searchQuery: search,
      manufacturersQuery: [],
      categoriesQuery: [],
    }),
  }).then((res) => {
    if (res.ok) {
      res.json().then((data: string[]) => {
        let i = data.map((d) => {
          const [manufacturer, name] = d.split("/");
          return { manufacturer, name };
        });
        availableFixtures = i;
      });
    } else {
      console.log(res);
    }
  });

  function importType(fixture: { manufacturer: string; name: string }) {
    fetch(
      "https://open-fixture-library.org/" +
        fixture.manufacturer +
        "/" +
        fixture.name +
        ".aglight",
      {
        headers: {},

        method: "GET",
        // mode: "cors",
      }
    )
      .then((res) => {
        if (res.ok) {
          res.json().then((data: string[]) => {
            console.log("imported");
            console.log(data);
            upload(JSON.stringify(data));
          });
        } else {
          console.log(res);
        }
      })
      .catch((err) => {
        console.log(err);
      });
  }
  function fitsSearch(
    fixture: { manufacturer: string; name: string },
    search: string
  ) {
    let i =
      fixture.manufacturer.toLowerCase() + "/" + fixture.name.toLowerCase();
    let keywords = search.split(" ");
    for (let keyword of keywords) {
      if (!i.includes(keyword.toLowerCase())) {
        return false;
      }
    }
    return true;
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="bg" on:click={() => close()}>
  <div
    class="inner"
    on:click={(e) => {
      e.stopPropagation();
    }}
  >
    <h3>Upload Fixture Definition</h3>
    <div class="tabs">
      <div
        class="tab {source === 'OFL' ? 'selected' : ''}"
        on:click={() => (source = "OFL")}
        role="button"
        tabindex={0}
        on:keypress
      >
        OFL
      </div>
      <div
        class="tab {source === 'JSON' ? 'selected' : ''}"
        on:click={() => (source = "JSON")}
        role="button"
        tabindex={0}
        on:keypress
      >
        JSON
      </div>
    </div>
    <div class="content">
      {#if source == "OFL"}
        <div class="searchbar">
          <input type="text" bind:value={search} />
        </div>
        <div class="results">
          {#if availableFixtures.length == 0}
            <p>No fixtures found. Is the Internet available?</p>
          {/if}
          {#each availableFixtures as fixture}
            {#if fitsSearch(fixture, search)}
              <div class="result">
                <p class="manufacturer">{fixture.manufacturer}</p>
                <p class="name">{fixture.name}</p>
                <input
                  type="button"
                  value="Import"
                  on:click={() => importType(fixture)}
                />
              </div>
            {/if}
          {/each}
        </div>
      {:else}
        <p>Paste AGLight JSON and click Upload</p>
        <textarea bind:this={inputJson} rows="35" cols="100" />
        <div class="btns">
          <button on:click={() => upload(inputJson.value)}>Upload</button>
          <button on:click={close}>Close</button>
        </div>
      {/if}
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
    background-color: #151111;
    border-radius: 0.25rem;
    padding: 1.5rem 2rem;
    display: grid;
    grid-template-rows: 3rem 2rem calc(100% - 5rem);
    width: 50vw;
    height: 80vh;
  }
  textarea {
    resize: none;
    width: 100%;
  }

  div.btns {
    display: flex;
    justify-content: flex-start;

    padding-top: 0.25rem;
  }
  div.btns button {
    margin-right: 0.5rem;
  }

  .tabs {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 2rem;
    border-bottom: #ff3e3e 1px solid;
  }
  .tab {
    width: 3rem;
    height: 100%;
    background-color: #333;
    color: #fff;
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: 0.5rem 0.5rem 0 0;
    cursor: pointer;
  }

  .selected {
    background-color: #ff3e3e;
    color: #fff;
  }

  .searchbar {
    display: flex;
    flex-direction: row;
    width: 100%;
    gap: 0.25rem;
    position: sticky;
    top: 0px;
    background-color: #151111;
    border-top: #ff3e3e 1px solid;
  }

  input[type="text"] {
    width: 100%;
    padding: 12px 20px;
    margin: 8px 0;
    box-sizing: border-box;
    border: none;
    border-bottom: 2px solid #ff3e3e;
  }

  input[type="button"] {
    background-color: #ff3e3e;
    border: none;
    color: white;
    padding: 16px 32px;
    text-decoration: none;
    margin: 4px 2px;
    cursor: pointer;
  }
  .results {
    overflow-y: scroll;
  }
  .content {
    overflow-y: scroll;
  }
  .result {
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 3rem;
    border-bottom: #ff3e3e 1px solid;
  }
  .manufacturer {
    width: 10%;
    padding-left: 0.5rem;
    font-size: small;
  }
  .name {
    width: 80%;
    padding-left: 0.5rem;
  }
</style>

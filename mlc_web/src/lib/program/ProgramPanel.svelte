<script lang="ts">
  import EffectBrowser from "./EffectBrowser.svelte";
  import { make_ws_uri } from "../stores";
  import type { Effect } from "../../customTypings/Effect";
  import {
    PanelLeftClose,
    PanelLeftOpen,
    Repeat,
    Repeat1,
  } from "lucide-svelte";
  import Timeline from "./timeline/Timeline.svelte";

  const socket = new WebSocket(make_ws_uri("/effects/effectHandler"));

  type EffectMessage =
    | { EffectCreated: { name: string; id: string } }
    | { EffectUpdated: { id: string } }
    | { EffectRunning: { id: string; running: boolean } }
    | {
        EffectList: { effects: [string, string][] };
      }
    | {
        Effect: { effect: Effect };
      };
  socket.onmessage = (data) => {
    let msg = JSON.parse(data.data) as EffectMessage;

    if ("Effect" in msg) {
      if (effect !== undefined) {
        console.log(JSON.stringify(effect.tracks));
        socket.send(
          '{"Update": {"id": "' +
            effect?.id +
            '", "tracks":' +
            JSON.stringify(effect?.tracks) +
            ', "looping": ' +
            effect.looping +
            ', "duration": ' +
            effect.duration +
            "}}"
        );
      }
      effect = msg.Effect.effect;
    }
  };

  let effect: Effect | undefined = undefined;

  let broswerOut: boolean = false;
</script>

<!--<EffectBrowser></EffectBrowser>-->
<!--<DualRingSpinner></DualRingSpinner>-->
<!--<PanTiltFeature></PanTiltFeature>-->
<!--<RgbFeature></RgbFeature>-->
<!--<Canvas>-->
<!--  <T.PerspectiveCamera-->
<!--    makeDefault-->
<!--    position={[10, 10, 10]}-->
<!--    on:create={({ ref }) => {-->
<!--      ref.lookAt(0, 1, 0);-->
<!--    }}-->
<!--  />-->
<!--  <Scene></Scene>-->
<!--</Canvas>-->

<div class="program">
  {#if broswerOut}
    <div class="panel effect-browser">
      <div class="header">
        <div
          class="iconBtn"
          on:click={() => (broswerOut = false)}
          on:keypress
          role="button"
          tabindex="0"
        >
          <PanelLeftClose size={"1.25rem"}></PanelLeftClose>
        </div>
        <h3>Effect Browser:</h3>
      </div>
      <EffectBrowser
        on:open={(v) => socket.send('{"Get": {"id": "' + v.detail + '"}}')}
      />
    </div>
  {:else}
    <div
      class="openEffectBrowser iconBtn"
      on:click={() => (broswerOut = true)}
      on:keypress
      role="button"
      tabindex="0"
    >
      <PanelLeftOpen size={"1.25rem"}></PanelLeftOpen>
    </div>
  {/if}
  <div class="panel timeline {broswerOut ? 'browser' : ''}">
    <!-- <h3>Timeline</h3> -->
    <Timeline></Timeline>
  </div>
  <div class="panel flat-preview {broswerOut ? 'browser' : ''}">
    <h3>Viewport:</h3>
  </div>
  <div class="panel effect-detail {broswerOut ? 'browser' : ''}">
    <h3>Effect:</h3>
    <div class="p-effect">
      <p>Name:</p>
      <p>{effect?.name}</p>
      <p>Looping:</p>
      <div class="looping">
        <button
          on:click={() => {
            if (effect) {
              effect.looping = !effect?.looping;
            }
          }}
        >
          {#if effect?.looping}
            <Repeat size={"1rem"}></Repeat>
          {:else}
            <Repeat1 size={"1rem"}></Repeat1>
          {/if}
        </button>
      </div>
      <p>Duration:</p>
      <p>{effect?.duration}s</p>
    </div>
  </div>
</div>

<style>
  .program {
    width: calc(100% - 1rem);
    height: calc(100% - 1rem);
    display: grid;
    padding: 0.5rem;

    grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
    grid-template-rows: 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
    gap: 0.5rem;

    min-height: 0;
    min-width: 0;
  }

  .panel {
    width: calc(100% - 2rem);
    padding: 0 1rem;
    border-radius: var(--number-border-radius);
    background-color: var(--color-panel);
    overflow: auto;
    min-width: 0;
    transition: 0.5s;
  }

  .header {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
    margin: 0;
    width: 100%;
  }

  .header div {
    cursor: pointer;
    display: grid;
    place-items: center;
    margin-top: 0.1rem;
  }

  .effect-browser {
    grid-column: 1 / 3;
    grid-row: 1 / 13;
  }

  .timeline.browser {
    grid-column: 3 / 13;
    grid-row: 7 / 13;
  }

  .flat-preview.browser {
    grid-column: 3 / 11;
    grid-row: 1 / 7;
  }

  .effect-detail.browser {
    grid-column: 11 / 13;
    grid-row: 1 / 7;
  }

  .timeline {
    grid-column: 1 / 13;
    grid-row: 7 / 13;
  }

  .flat-preview {
    grid-column: 1 / 9;
    grid-row: 1 / 7;
  }

  .effect-detail {
    grid-column: 9 / 13;
    grid-row: 1 / 7;
  }

  .panel h3 {
    text-align: center;
    margin: 0;
    width: 100%;
    -webkit-touch-callout: none;
    -webkit-user-select: none;
    -khtml-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
    position: sticky;
    top: 0px;
    background-color: var(--color-panel);
    padding-top: 0.25rem;
    z-index: 15;
  }

  .openEffectBrowser {
    position: absolute;
    left: 0;
    top: 3.5rem;
    display: grid;
    place-items: center;
    cursor: pointer;
    padding: 0.5rem;
    border-top-right-radius: var(--number-border-radius);
    border-bottom-right-radius: var(--number-border-radius);
    background-color: var(--color-panel);
    z-index: 18;
    box-shadow: 0 0 0.2rem 0.2rem #000000c4;
  }

  .iconBtn:hover {
    cursor: pointer;
    color: var(--color-accent);
  }

  .p-effect {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.2rem;
  }

  .p-effect p {
    margin: 0;
  }
  .p-effect .looping {
    display: grid;
    place-items: center;
  }
  .p-effect .looping button {
    padding: 0.2rem 0.5rem;
    margin: 0;
  }
</style>

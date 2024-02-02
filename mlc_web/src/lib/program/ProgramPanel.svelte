<script lang="ts">
    import EffectBrowser from "./EffectBrowser.svelte";
    import {make_ws_uri} from "../stores";
    import type {Effect} from "../../customTypings/Effect";


    const socket = new WebSocket(make_ws_uri("/effects/effectHandler"));

    type EffectMessage =
        { "EffectCreated": { "name": string, "id": string } }
        | { "EffectUpdated": { "id": string } }
        | { "EffectRunning": { "id": string, "running": boolean } }
        | {
        "EffectList": { "effects": [string, string][] }
    } | {
        "Effect": { "effect": Effect }
    };
    socket.onmessage = (data) => {
        let msg = JSON.parse(data.data) as EffectMessage;

        if ("Effect" in msg) {
            if (effect !== undefined) {
                console.log(JSON.stringify(effect.tracks));
                socket.send('{"Update": {"id": "' + effect?.id + '", "tracks":' + JSON.stringify(effect?.tracks) + ', "looping": ' + effect.looping + ', "duration": ' + effect.duration + '}}')
            }
            effect = msg.Effect.effect;
        }
    }

    let effect: Effect | undefined = undefined;

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
    <div class="panel effect-browser"><h3>Effect Browser:</h3>
        <EffectBrowser on:open={(v) => socket.send('{"Get": {"id": "'+v.detail+'"}}')}/>
    </div>
    <div class="panel timeline"><h3>Timeline</h3></div>
    <div class="panel flat-preview"><h3>Viewport:</h3></div>
    <div class="panel effect-detail"><h3>Effect:</h3>
        <p>Name: {effect?.name}</p>
        <p>Looping: {effect?.looping}</p>
        <p>Duration: {effect?.duration}s</p>
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
    }

    .effect-browser {
        grid-column: 1 / 3;
        grid-row: 1 / 13;
    }

    .timeline {
        grid-column: 3 / 13;
        grid-row: 7 / 13;
    }

    .flat-preview {
        grid-column: 3 / 11;
        grid-row: 1 / 7;
    }

    .effect-detail {
        grid-column: 11 / 13;
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
</style>

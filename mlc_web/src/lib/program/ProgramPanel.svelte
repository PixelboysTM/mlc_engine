<script lang="ts">
  import { Canvas } from "@threlte/core";
  import Scene from "./Scene.svelte";
  import { T } from "@threlte/core";
  import DualRingSpinner from "../misc/DualRingSpinner.svelte";
  import PanTiltFeature from "../configure/fixtureTester/PanTiltFeature.svelte";
  import RgbFeature from "../configure/fixtureTester/RgbFeature.svelte";
</script>

<DualRingSpinner></DualRingSpinner>
<PanTiltFeature></PanTiltFeature>
<RgbFeature></RgbFeature>
<Canvas>
  <T.PerspectiveCamera
    makeDefault
    position={[10, 10, 10]}
    on:create={({ ref }) => {
      ref.lookAt(0, 1, 0);
    }}
  />
  <Scene></Scene>
</Canvas>

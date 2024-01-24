<script lang="ts">
  import { Canvas } from "@threlte/core";
  import Scene from "./Scene.svelte";
  import { T } from "@threlte/core";
</script>

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

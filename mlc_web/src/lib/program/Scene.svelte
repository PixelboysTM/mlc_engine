<script lang="ts">
  import { T, useTask } from "@threlte/core";
  import { spring } from "svelte/motion";
  import { interactivity } from "@threlte/extras";

  interactivity();
  const scale = spring(1);
  let rotation = 0;
  useTask((delta) => {
    rotation += delta;
  });
</script>

<T.DirectionalLight position={[10, 10, 0]} intensity={0.5} castShadow />
<T.Mesh
  rotation.y={rotation}
  position.y={1}
  scale={$scale}
  on:pointerenter={() => scale.set(1.5)}
  on:pointerleave={() => scale.set(1)}
>
  <T.BoxGeometry />
  <T.MeshStandardMaterial color="red" />
</T.Mesh>

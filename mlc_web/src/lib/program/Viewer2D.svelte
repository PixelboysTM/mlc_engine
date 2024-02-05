<script lang="ts">
  import P5 from "p5-svelte";
  import type { Sketch } from "p5-svelte";

  const sketch: Sketch = (p5) => {
    p5.setup = () => {
      p5.createCanvas(400, 400);
      p5.background(0);
    };
    p5.draw = () => {
      if (
        p5.mouseX > 0 &&
        p5.mouseX < p5.width &&
        p5.mouseY > 0 &&
        p5.mouseY < p5.height &&
        p5.mouseIsPressed
      ) {
        p5.fill(255, 0, 0);
      } else {
        p5.fill(255);
      }

      p5.ellipse(p5.width / 2, p5.height / 2, 50, 50);
    };
  };
</script>

<P5 {sketch} />

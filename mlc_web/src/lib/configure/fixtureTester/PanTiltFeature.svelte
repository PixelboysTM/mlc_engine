<script lang="ts">
  import { createSlider, melt } from "@melt-ui/svelte";
  import { writable } from "svelte/store";
  import Icon from "svelte-icons-pack/Icon.svelte";
  import RiSystemAddLine from "svelte-icons-pack/ri/RiSystemAddLine";
  import { createEventDispatcher } from "svelte";

  let valueX = writable([0.5]);
  let valueY = writable([0.5]);

  let canvas: HTMLCanvasElement;
  let thumbSize = 15;

  const dispatcher = createEventDispatcher<Record<string, number[]>>();

  $: {
    if (canvas) {
      draw($valueX[0], $valueY[0]);
    }
  }

  function draw(x: number, y: number) {
    function capX(v: number): number {
      return Math.min(Math.max(v, thumbSize + 2), canvas.width - thumbSize - 2);
    }
    function capY(v: number): number {
      return Math.min(
        Math.max(v, thumbSize + 2),
        canvas.height - thumbSize - 2
      );
    }

    console.log("Updating");

    dispatcher("value", [x, y]);

    const ctx = canvas.getContext("2d");

    if (ctx) {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      //BG
      ctx.fillStyle =
        getComputedStyle(canvas).getPropertyValue("--color-panel");
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      // X Line
      ctx.strokeStyle =
        getComputedStyle(canvas).getPropertyValue("--color-background");
      ctx.lineWidth = 5;
      ctx.beginPath();
      ctx.moveTo(0, capY(canvas.height * y));
      ctx.lineTo(canvas.width, capY(canvas.height * y));
      ctx.stroke();

      // Y Line
      ctx.beginPath();
      ctx.moveTo(capX(canvas.width * x), 0);
      ctx.lineTo(capX(canvas.width * x), canvas.height);
      ctx.stroke();

      // Thumb
      ctx.fillStyle =
        getComputedStyle(canvas).getPropertyValue("--color-accent");
      ctx.beginPath();
      ctx.ellipse(
        capX(canvas.width * x),
        capY(canvas.height * y),
        thumbSize,
        thumbSize,
        0,
        0,
        2 * Math.PI,
        false
      );
      ctx.fill();
    }
  }

  function touchMove(event: TouchEvent) {
    const rect = canvas.getBoundingClientRect();
    const x = event.touches[0].clientX - rect.left;
    const y = event.touches[0].clientY - rect.top;

    let vX = x / rect.width;
    let vY = y / rect.height;
    valueX.set([vX]);
    valueY.set([vY]);
    // draw(vX, vY);

    event.preventDefault();
  }

  function mouseMove(event: MouseEvent) {
    console.log(event.buttons);
    if (event.buttons != 1) return;

    const rect = canvas.getBoundingClientRect();
    const x = event.clientX - rect.left;
    const y = event.clientY - rect.top;

    let vX = x / rect.width;
    let vY = y / rect.height;
    valueX.set([vX]);
    valueY.set([vY]);
    // draw(vX, vY);
  }

  const sliderX = createSlider({
    defaultValue: [0],
    min: 0,
    max: 1,
    step: 0.001,
    value: valueX,
  });
  let sliderXRange = sliderX.elements.range;
  let sliderXRoot = sliderX.elements.root;
  let sliderXThumb = sliderX.elements.thumbs;

  const sliderY = createSlider({
    defaultValue: [0],
    min: 0,
    max: 1,
    step: 0.001,
    value: valueY,
    orientation: "vertical",
    dir: "rtl",
  });
  let sliderYRange = sliderY.elements.range;
  let sliderYRoot = sliderY.elements.root;
  let sliderYThumb = sliderY.elements.thumbs;
</script>

<h3>Pan / Tilt</h3>
<div class="slider">
  <div class="box">
    <canvas
      class="ctx"
      width="500"
      height="500"
      bind:this={canvas}
      on:mousemove={mouseMove}
      on:touchmove={touchMove}
    ></canvas>
  </div>
  <span class="sliderX sliderC" use:melt={$sliderXRoot}>
    <span class="outerX">
      <span class="rangeX" use:melt={$sliderXRange}> </span>
    </span>
    <span class="thumb" use:melt={$sliderXThumb[0]}></span>
  </span>
  <span class="sliderY sliderC" use:melt={$sliderYRoot}>
    <span class="outerY">
      <span class="rangeY" use:melt={$sliderYRange}> </span>
    </span>
    <span class="thumb" use:melt={$sliderYThumb[0]}></span>
  </span>
  <div class="corner">
    <Icon src={RiSystemAddLine} color={"var(--color-text)"} size={"100%"}
    ></Icon>
  </div>
</div>
<p class="value">
  Pan:{" " + $valueX[0].toFixed(2)} / Tilt:{" " + $valueY[0].toFixed(2)}
</p>

<style>
  h3 {
    margin: 0;
    margin-bottom: 0.25rem;
    text-align: center;
  }
  .value {
    margin-left: auto;
    margin-right: auto;
    margin-bottom: 0.2rem;
    margin-top: 0.2rem;
    text-align: center;
  }
  .corner {
    grid-area: 2 / 2 / 2 / 2;
    background-color: var(--color-panel);
    border-radius: var(--number-border-radius);
    display: flex;
    justify-content: center;
    align-items: center;
  }
  .sliderC {
    background-color: var(--color-panel);
    border-radius: var(--number-border-radius);
  }
  .sliderX {
    grid-area: 2 / 1 / 2 / 1;
    position: relative;
    display: flex;
    width: 100%;
    height: 100%;
    align-items: center;
  }
  .sliderY {
    grid-area: 1 / 2 / 1 / 2;
    position: relative;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    align-items: center;
  }

  .outerX {
    height: 3px;
    width: 100%;
    background-color: var(--color-background);
  }

  .rangeX {
    height: 3px;
    /* background-color: var(--color-text); */
  }

  .outerY {
    width: 3px;
    height: 100%;
    background-color: var(--color-background);
  }

  .rangeY {
    width: 3px;
    /* background-color: var(--color-text); */
  }

  .thumb {
    height: 10px;
    width: 10px;
    background-color: var(--color-text);
    border-radius: 50%;
  }

  .slider {
    display: grid;
    grid-template-columns: 1fr 2rem;
    grid-template-rows: 1fr 2rem;

    background-color: var(--color-background);
    border: 1px solid var(--color-accent);
    border-radius: var(--number-border-radius);
    gap: 0.25rem;

    padding: 0.25rem;
  }

  .box {
    width: 12rem;
    height: 12rem;

    grid-area: 1 / 1 / 1 / 1;
  }

  .ctx {
    width: 100%;
    height: 100%;
    border-radius: var(--number-border-radius);
  }
</style>

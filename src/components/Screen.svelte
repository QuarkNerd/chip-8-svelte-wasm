<script lang="typescript">
  import { offColour, onColour } from "../stores.js";
  import { onMount } from "svelte";
  export let displayArray = new Uint8Array();

  const COLS: number = 64;
  const ROWS: number = 32;
  const scale: number = 5;

  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  onMount(() => {
    ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
    canvas.width = scale * COLS;
    canvas.height = scale * ROWS;
    ctx.fillStyle = $offColour;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  });

  export function draw() {
    if (ctx) {
      resetScreen();
      ctx.fillStyle = $onColour;
      displayArray.forEach((on: number, i: number) => {
        if (on) {
          const x = i % COLS;
          const y = Math.floor(i / COLS);
          ctx.fillRect(x * scale, y * scale, scale, scale);
        }
      });
    }
  }

  export function resetScreen() {
    ctx.fillStyle = $offColour;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  }
</script>

<canvas bind:this={canvas} />

<style>
  canvas {
    border: 2px solid black;
  }
</style>

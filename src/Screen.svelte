<script lang="typescript">
  import { onMount } from "svelte";
  export let onColour: string;
  export let offColour: string;
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
    ctx.fillStyle = offColour;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  });

  export function draw() {
    if (ctx) {
      ctx.fillStyle = offColour;
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      ctx.fillStyle = onColour;
      displayArray.forEach((on: number, i: number) => {
        if (on) {
          const x = i % COLS;
          const y = Math.floor(i / COLS);
          ctx.fillRect(x * scale, y * scale, scale, scale);
        }
      });
    }
  }
</script>

<main>
  <canvas bind:this={canvas} />
</main>

<style>
  canvas {
    border: 2px solid black;
  }
</style>

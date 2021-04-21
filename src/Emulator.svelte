<script lang="typescript">
  import { onMount } from "svelte";
  
  const COLS: number = 64;
  const ROWS: number = 32;
  
  export let scale: number;
  let canvas: HTMLCanvasElement;

  onMount(() => {
    const ctx = canvas.getContext("2d");
    canvas.width = scale * COLS;
    canvas.height = scale * ROWS;
  });
</script>

<main>
  <canvas bind:this={canvas} />
</main>

<style>
  canvas {
    border: 2px solid black;
  }
</style>

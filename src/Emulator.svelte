<script lang="typescript">
  import { onMount } from "svelte";
  import wasm from "./wasm/Cargo.toml";
  
  const COLS: number = 64;
  const ROWS: number = 32;
  
  async function loadWasm() {
		const wasmModule = await wasm();
    const wasmEmulator = new wasmModule.Emulator();
    const displayArray = wasmEmulator.get_display();
    console.log(displayArray);
	}

  loadWasm();

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

<script lang="typescript">
  import Screen from "./Screen.svelte";
  import wasm from "./wasm/Cargo.toml";

  let displayArray: Uint8Array;

  loadWasm();

  async function loadWasm() {
    const wasmModule = await wasm();
    const wasmEmulator = new wasmModule.Emulator();
    displayArray = wasmEmulator.get_display();
  }
</script>

<main>
  <Screen colour="#000" scale={5} displayArray={displayArray}/>
</main>

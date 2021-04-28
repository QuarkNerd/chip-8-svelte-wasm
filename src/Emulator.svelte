<script lang="typescript">
  import Screen from "./Screen.svelte";
  import Keyboard from "./Keyboard.svelte";
  import wasm from "./wasm/Cargo.toml";

  let wasmEmulator: any;
  let keysArray = new Uint8Array(0x10); 
  let displayArray: Uint8Array;

  loadWasm();

  async function loadWasm() {
    const wasmModule = await wasm();
    wasmEmulator = new wasmModule.Emulator();
    displayArray = wasmEmulator.get_display();
    keysArray = wasmEmulator.get_keys();
  }
</script>

<main>
  <Screen colour="#000" scale={5} {displayArray} />
  <Keyboard bind:keysArray={keysArray}></Keyboard>
</main>

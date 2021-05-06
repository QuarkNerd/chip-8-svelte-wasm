<script lang="typescript">
  import Screen from "./Screen.svelte";
  import Keyboard from "./Keyboard.svelte";
  import wasm from "./wasm/Cargo.toml";

  let wasmEmulator: any;
  let keysArray = new Uint8Array(0x10);
  let displayArray: Uint8Array;
  let loop: number;
  let screen: Screen;
  loadWasm();

  async function loadWasm() {
    const wasmModule = await wasm();
    wasmEmulator = new wasmModule.Emulator();
    displayArray = wasmEmulator.get_display();
    keysArray = wasmEmulator.get_keys();
    loadRom();
  }

  async function loadRom() {
    let response = await fetch("roms/BLITZZ");
    let arrayBuffer = await response.arrayBuffer();
    let rom = new Uint8Array(arrayBuffer);
    wasmEmulator.load_rom(rom);
    loop = requestAnimationFrame(runEmulator);
  }

  function runEmulator() {
    wasmEmulator.on_animation_frame();
    screen.draw();
    loop = requestAnimationFrame(runEmulator);
  }
</script>

<main>
  <Screen bind:this={screen} colour="#000" scale={5} {displayArray} />
  <Keyboard bind:keysArray />
</main>

<script lang="typescript">
  import Screen from "./Screen.svelte";
  import Keyboard from "./Keyboard.svelte";
  import Speaker from "./Speaker.svelte";
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
    let response = await fetch("roms/BLITZ");
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
  <div class="screen-frame">
    <Screen bind:this={screen} offColour="#9aa040" onColour="#000000" {displayArray} />
  </div>
  <Keyboard bind:keysArray />
</main>

<style>
  main {
    padding: 10px 0;
    margin: 0;
    width: 450px;
    border-radius: 35px;
    background-color: lightgray;
    box-shadow:
      rgb(0 0 0) -4px 6px 10px, rgb(255 255 255 / 25%) -6px 8px 6px inset;
  }

  .screen-frame {
    height: 160px;
    width: 320px;
    background-color: darkgray;
    margin: 20px auto;
    padding: 30px 30px;
    border-radius: 45px 25px 0px;
  }
</style>
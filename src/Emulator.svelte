<script lang="typescript">
  import Screen from "./Screen.svelte";
  import Keyboard from "./Keyboard.svelte";
  import Speaker from "./Speaker.svelte";
  import GameSlot from "./GameSlot.svelte";
  import GameCartriage from "./GameCartriage.svelte";
  import wasm from "./wasm/Cargo.toml";

  export let selectedGame:
    | {
        name: string;
        colour: string;
      }
    | undefined;

  let wasmEmulator: any;
  let keysArray = new Uint8Array(0x10);
  let displayArray: Uint8Array;
  let loop: number;
  let screen: Screen;
  
  loadWasm();
  $: loadRom(selectedGame?.name);

  async function loadWasm() {
    const wasmModule = await wasm();
    wasmEmulator = new wasmModule.Emulator();
    displayArray = wasmEmulator.get_display();
    keysArray = wasmEmulator.get_keys();
    loadRom(selectedGame?.name);
  }

  async function loadRom(game: string | undefined) {
    cancelAnimationFrame(loop);
    screen.resetScreen();
    if (!game) return;
    let response = await fetch(`roms/${game}`);
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
    <Screen
      bind:this={screen}
      offColour="#9aa040"
      onColour="#000000"
      {displayArray}
    />
  </div>
  <Keyboard bind:keysArray />
  <div class="speaker">
    <Speaker />
  </div>
  <div class="gameslot">
    <GameSlot />
    {#if selectedGame}
      <GameCartriage on:gameClicked {...selectedGame} />
    {/if}
  </div>
</main>

<style>
  main {
    position: relative;
    padding: 10px 0;
    margin: 0 5px;
    width: 450px;
    border-radius: 35px;
    flex-shrink: 0;

    background-color: lightgray;
    box-shadow: rgb(0 0 0) -4px 6px 10px,
      rgb(255 255 255 / 25%) -6px 8px 6px inset;
  }

  .screen-frame {
    height: 160px;
    width: 320px;
    background-color: darkgray;
    margin: 20px auto;
    padding: 30px 30px;
    border-radius: 45px 25px 0px;
  }

  .speaker {
    position: absolute;
    left: 340px;
    top: 300px;
  }

  .gameslot {
    position: absolute;
    left: 20px;
    top: 280px;
  }
</style>

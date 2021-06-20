<script lang="typescript">
  import { flip } from "svelte/animate";

  import Screen from "./Screen.svelte";
  import Keyboard from "./Keyboard.svelte";
  import Speaker from "./Speaker.svelte";
  import GameSlot from "./GameSlot.svelte";
  import GameCartriage from "./GameCartriage.svelte";
  import SlidingButton from "./SlidingButton.svelte";

  import type { Game, Transition } from "../types";
  import { gameSpeed } from "../stores";
  import wasm from "./../wasm/Cargo.toml";

  export let selectedGame: Game | undefined;
  export let cartriageTransition: Transition;

  let wasmEmulator: any;
  let keysArray = new Uint8Array(0x10);
  let displayArray: Uint8Array;
  let soundArray: Uint8Array = new Uint8Array(1);
  let loop: number;
  let screen: Screen;
  let playing: boolean = false;
  let soundOn: boolean = false;
  loop = requestAnimationFrame(runEmulator);

  gameSpeed.subscribe((speed: number) => wasmEmulator?.set_speed(speed));

  loadWasm();
  $: loadRom(selectedGame);

  async function loadWasm() {
    const wasmModule = await wasm();
    wasmEmulator = new wasmModule.Emulator();
    wasmEmulator.set_speed($gameSpeed);
    displayArray = wasmEmulator.get_display();
    keysArray = wasmEmulator.get_keys();
    soundArray = wasmEmulator.get_sound();
    loadRom(selectedGame);
  }

  async function loadRom(game: Game | undefined) {
    screen?.resetScreen();
    if (!game) return (playing = false);
    let response = await fetch(`roms/${game.name}`);
    let arrayBuffer = await response.arrayBuffer();
    let rom = new Uint8Array(arrayBuffer);
    wasmEmulator.load_rom(rom, game.yWrap);
    playing = true;
  }

  function runEmulator() {
    if (!(playing && selectedGame))
      return (loop = requestAnimationFrame(runEmulator));
    wasmEmulator.on_animation_frame();
    screen.draw();
    loop = requestAnimationFrame(runEmulator);
    soundOn = soundArray[0] > 0;
  }
  const send = cartriageTransition.send;
  const receive = cartriageTransition.receive;
  let gameArray: Game[] = [];
  $: gameArray = selectedGame ? [selectedGame] : [];
</script>

<div class="Emulator">
  <div class="screen-frame">
    <Screen bind:this={screen} {displayArray} />
  </div>
  <Keyboard bind:keysArray />
  <div class="speaker">
    <Speaker on={soundOn} />
  </div>
  <div class="gameslot">
    <GameSlot />
  </div>
  {#each gameArray as game (game.name)}
    <div
      in:receive={{ key: game.name }}
      out:send={{ key: game.name }}
      animate:flip
      class="gameslot"
    >
      <GameCartriage on:gameClicked {game} />
    </div>
  {/each}

  <div class="pause">
    <SlidingButton bind:active={playing} />
  </div>
</div>

<style>
  .Emulator {
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

  .pause {
    position: absolute;
    left: 20px;
    top: 330px;
  }
</style>

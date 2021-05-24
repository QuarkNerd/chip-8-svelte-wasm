<script lang="ts">
  import Emulator from "./Emulator.svelte";
  import GamesHolder from "./GamesHolder.svelte";

  const games = [
    "15PUZZLE",
    "BLINKY",
    "BLITZ",
    "BRIX",
    "CONNECT4",
    "GUESS",
    "HIDDEN",
    "IBM",
    "INVADERS",
    "KALEID",
    "MAZE",
    "MERLIN",
    "MISSILE",
    "PONG",
    "PONG2",
    "PUZZLE",
    "SYZYGY",
    "TANK",
    "TETRIS",
    "TICTAC",
    "UFO",
    "VBRIX",
    "VERS",
    "WIPEOFF",
  ];
</script>

<main>
  <Emulator />
  <GamesHolder {games} />
</main>

<style>
  main {
    display: flex;
    flex-direction: row;
    justify-content: center;
    margin: 10px auto;
  }
</style>

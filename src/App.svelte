<script lang="ts">
  import Emulator from "./Emulator.svelte";
  import GamesHolder from "./GamesHolder.svelte";

  import type { Game } from  './types'

  const games: Game[] = [
    { name: "15PUZZLE", colour: "blue" },
    { name: "BLINKY", colour: "blue" },
    { name: "BLITZ", colour: "blue" },
    { name: "BRIX", colour: "blue" },
    { name: "CONNECT4", colour: "blue" },
    { name: "GUESS", colour: "blue" },
    { name: "HIDDEN", colour: "blue" },
    { name: "IBM", colour: "blue" },
    { name: "INVADERS", colour: "blue" },
    { name: "KALEID", colour: "blue" },
    { name: "MAZE", colour: "blue" },
    { name: "MERLIN", colour: "blue" },
    { name: "MISSILE", colour: "blue" },
    { name: "PONG", colour: "blue" },
    { name: "PONG2", colour: "blue" },
    { name: "PUZZLE", colour: "blue" },
    { name: "SYZYGY", colour: "blue" },
    { name: "TANK", colour: "blue" },
    { name: "TETRIS", colour: "blue" },
    { name: "TICTAC", colour: "blue" },
    { name: "UFO", colour: "blue" },
    { name: "VBRIX", colour: "blue" },
    { name: "VERS", colour: "blue" },
    { name: "WIPEOFF", colour: "blue" },
  ];

  let selectedGameName: string | null = "WIPEOFF";

  $: selectedGame = games.find((g) => g.name === selectedGameName);
  $: remaingGames = games.filter((g) => g.name !== selectedGameName);

  const gameClicked = (event: any) =>
    (selectedGameName =
      event.detail.game === selectedGameName ? null : event.detail.game);
</script>

<main>
  <Emulator on:gameClicked={gameClicked} {selectedGame} />
  <GamesHolder on:gameClicked={gameClicked} games={remaingGames} />
</main>

<style>
  main {
    display: flex;
    flex-direction: row;
    justify-content: center;
    margin: 10px auto;
  }
</style>

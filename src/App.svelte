<script lang="ts">
  import Emulator from "./Emulator.svelte";
  import GamesHolder from "./GamesHolder.svelte";

  import type { Game, Transition } from "./types";

  import { quintOut } from "svelte/easing";
  import { crossfade } from "svelte/transition";

  const GAME_CHOOSE_DURATION: number = 700;
  let is_game_changing = false;

  const [send, receive] = crossfade({
    duration: (_) => GAME_CHOOSE_DURATION,

    fallback(node, _, __) {
      const style = getComputedStyle(node);
      const transform = style.transform === "none" ? "" : style.transform;

      return {
        duration: 600,
        easing: quintOut,
        css: (t) => `
  				transform: ${transform} scale(${t});
  				opacity: ${t}
  			`,
      };
    },
  });

  const cartriageTransition: Transition = {
    send,
    receive,
  };

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

  const gameClicked = (event: any) => {
    if (is_game_changing) return;
    is_game_changing = true;
    selectedGameName =
      event.detail.game === selectedGameName ? null : event.detail.game;
    setTimeout(() => (is_game_changing = false), GAME_CHOOSE_DURATION);
  };
</script>

<main>
  <Emulator {cartriageTransition} on:gameClicked={gameClicked} {selectedGame} />
  <GamesHolder
    {cartriageTransition}
    on:gameClicked={gameClicked}
    games={remaingGames}
  />
</main>

<style>
  main {
    display: flex;
    flex-direction: row;
    justify-content: center;
    margin: 10px auto;
  }
</style>

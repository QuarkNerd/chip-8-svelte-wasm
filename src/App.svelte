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
    { name: "15PUZZLE", colour: getRandomColour() },
    { name: "BLINKY", colour: getRandomColour() },
    { name: "BLITZ", colour: getRandomColour() },
    { name: "BRIX", colour: getRandomColour() },
    { name: "CONNECT4", colour: getRandomColour() },
    { name: "GUESS", colour: getRandomColour() },
    { name: "HIDDEN", colour: getRandomColour() },
    { name: "IBM", colour: getRandomColour() },
    { name: "INVADERS", colour: getRandomColour() },
    { name: "KALEID", colour: getRandomColour() },
    { name: "MAZE", colour: getRandomColour() },
    { name: "MERLIN", colour: getRandomColour() },
    { name: "MISSILE", colour: getRandomColour() },
    { name: "PONG", colour: getRandomColour() },
    { name: "PONG2", colour: getRandomColour() },
    { name: "PUZZLE", colour: getRandomColour() },
    { name: "SYZYGY", colour: getRandomColour() },
    { name: "TANK", colour: getRandomColour() },
    { name: "TETRIS", colour: getRandomColour() },
    { name: "TICTAC", colour: getRandomColour() },
    { name: "UFO", colour: getRandomColour() },
    { name: "VBRIX", colour: getRandomColour() },
    { name: "VERS", colour: getRandomColour() },
    { name: "WIPEOFF", colour: getRandomColour() },
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

  function getRandomColour() {
    const colours = ["green", "red", "darkorange", "purple", "blue"];
    const index = Math.floor(Math.random()*colours.length);
    return colours[index];
  }
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

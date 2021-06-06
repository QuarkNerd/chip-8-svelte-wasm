<script lang="ts">
  import Emulator from "./Emulator.svelte";
  import GamesHolder from "./GamesHolder.svelte";
  import OptionsModal from "./OptionsModal.svelte";

  import type { Game, Transition } from "../types";

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
    { name: "15PUZZLE", colour: getRandomColour(), yWrap: true },
    { name: "BLINKY", colour: getRandomColour(), yWrap: true },
    { name: "BLITZ", colour: getRandomColour(), yWrap: false },
    { name: "BRIX", colour: getRandomColour(), yWrap: true },
    { name: "CONNECT4", colour: getRandomColour(), yWrap: true },
    { name: "GUESS", colour: getRandomColour(), yWrap: true },
    { name: "HIDDEN", colour: getRandomColour(), yWrap: true },
    { name: "IBM", colour: getRandomColour(), yWrap: true },
    { name: "INVADERS", colour: getRandomColour(), yWrap: true },
    { name: "KALEID", colour: getRandomColour(), yWrap: true },
    { name: "MAZE", colour: getRandomColour(), yWrap: true },
    { name: "MERLIN", colour: getRandomColour(), yWrap: true },
    { name: "MISSILE", colour: getRandomColour(), yWrap: true },
    { name: "PONG", colour: getRandomColour(), yWrap: true },
    { name: "PONG2", colour: getRandomColour(), yWrap: true },
    { name: "PUZZLE", colour: getRandomColour(), yWrap: true },
    { name: "SYZYGY", colour: getRandomColour(), yWrap: true },
    { name: "TANK", colour: getRandomColour(), yWrap: true },
    { name: "TETRIS", colour: getRandomColour(), yWrap: true },
    { name: "TICTAC", colour: getRandomColour(), yWrap: true },
    { name: "UFO", colour: getRandomColour(), yWrap: true },
    { name: "VBRIX", colour: getRandomColour(), yWrap: true },
    { name: "VERS", colour: getRandomColour(), yWrap: true },
    { name: "WIPEOFF", colour: getRandomColour(), yWrap: true },
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
    const index = Math.floor(Math.random() * colours.length);
    return colours[index];
  }
</script>

<main>
  <OptionsModal open={true} />
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

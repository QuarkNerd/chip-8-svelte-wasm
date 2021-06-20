<script lang="typescript">
  import { createEventDispatcher } from "svelte";
  import type { Game } from "../types";

  export let game: Game;

  const dispatch = createEventDispatcher();

  function gameClicked() {
    dispatch("gameClicked", {
      game: game.name,
    });
  }
</script>

<div
  class="game-cartriage"
  on:click={gameClicked}
  style="background: {game.colour};"
>
  <div class="logo">C8</div>
  <div class="name">{game.name}</div>
</div>

<style>
  .game-cartriage {
    margin: 0 auto;

    height: 35px;
    width: 82px;
    border-radius: 0 4px 4px 15px;

    box-shadow: rgb(0 0 0) -2px 3px 5px,
      rgb(255 255 255 / 25%) -3px 4px 3px inset;
    text-align: center;

    display: flex;
    flex-direction: row;

    cursor: default;
  }

  .logo {
    margin: 5px 5px;
    padding: 0 5px;
    border-radius: 5px;
    background-color: black;

    font-family: Chalkduster, fantasy;
    color: white;
    line-height: 25px;
    font-size: 0.8rem;
  }

  .name {
    color: aliceblue;
    font-size: 0.5rem;
    font-weight: bold;
    line-height: 35px;
  }
</style>

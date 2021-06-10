<script lang="typescript">
  import { flip } from "svelte/animate";

  import GameCartriage from "./GameCartriage.svelte";
  import type { Game, Transition } from "../types";

  export let games: Game[];
  export let cartriageTransition: Transition;

  const send = cartriageTransition.send;
  const receive = cartriageTransition.receive;
</script>

<div class="games-holder">
  {#each games as game (game.name)}
    <div
      in:receive={{ key: game.name }}
      out:send={{ key: game.name }}
      animate:flip
    >
      <GameCartriage on:gameClicked {game} />
    </div>
  {/each}
</div>

<style>
  .games-holder {
    position: relative;
    margin: 0 5px;
    padding: 20px 0;
    height: 400px;
    width: 280px;
    border-radius: 35px;

    background-color: lightgray;
    box-shadow: rgb(0 0 0) -4px 6px 10px,
      rgb(255 255 255 / 25%) -6px 8px 6px inset;

    overflow: hidden;

    display: flex;
    flex-flow: row wrap;
    flex-shrink: 0;
    justify-content: space-around;
  }
</style>

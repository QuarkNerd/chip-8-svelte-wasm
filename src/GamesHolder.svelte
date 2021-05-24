<script lang="typescript">
  import GameCartriage from "./GameCartriage.svelte";
  export let games: string[] = [];
</script>

<main>
  {#each games as game}
    <GameCartriage name={game} colour="blue"/>
  {/each}
</main>

<style>
  main {
    position: relative;
    margin: 0 5px;
    padding: 20px 0;
    height: 400px;
    width: 280px;
    border-radius: 35px;
    flex-shrink: 0;

    background-color: lightgray;
    box-shadow: rgb(0 0 0) -4px 6px 10px,
      rgb(255 255 255 / 25%) -6px 8px 6px inset;

    overflow: hidden;

    display: flex;
    flex-flow: row wrap;
  }
</style>

<script lang="typescript">
  export let name: string;
  export let colour: string;
</script>

<main style="background: {colour};">
  <div class="logo">C8</div>
  <div class="name">{name}</div>
</main>

<style>
  main {
    margin: 0 auto;

    height: 35px;
    width: 82px;
    border-radius: 0 4px 4px 15px;

    box-shadow: rgb(0 0 0) -2px 3px 5px, rgb(255 255 255 / 25%) -3px 4px 3px inset;
    text-align: center;

    display: flex;
    flex-direction: row;
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
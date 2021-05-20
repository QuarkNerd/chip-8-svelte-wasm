<main>
  <div class="band"></div>
  <div class="band"></div>
  <div class="band"></div>
  <div class="band"></div>
</main>

<style>
  main {
    display: flex;
    flex-direction: row;
  }

  .band {
    margin-left: 8px;
    transform: rotate(40deg);
    height: 60px;
    width: 8px;
    border-radius: 8px;
    background: rgba(0, 0, 0, 0.35);
    box-shadow: rgb(0 0 0 / 60%) 3px 6px 1px inset;
  }
</style>

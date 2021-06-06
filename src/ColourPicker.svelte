<script lang="typescript">
  import {
    offColour,
    onColour,
    resetOffColour,
    resetOnColour,
  } from "./stores.js";
</script>

<div>
  <div class="selector">
    <label for="off-colour">Screen-off colour</label>
    <input
      type="color"
      id="off-colour"
      name="off-colour"
      bind:value={$offColour}
    />
    <button on:click={resetOffColour}>Reset</button>
  </div>
  <div class="selector">
    <label for="on-colour">Screen-on colour </label>
    <input
      type="color"
      id="on-colour"
      name="on-colour"
      bind:value={$onColour}
    />
    <button on:click={resetOnColour}>Reset</button>
  </div>
</div>

<style>
  .selector {
    position: relative;
    margin: 5px;
    height: 27px;

    display: flex;
    justify-content: space-between;
  }

  button {
    height: 100%;
  }
</style>

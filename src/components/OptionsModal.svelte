<script lang="typescript">
  import {
    offColour,
    onColour,
    gameSpeed,
    DEFAULT_OFF_COLOUR,
    DEFAULT_ON_COLOUR,
    DEFAULT_GAME_SPEED,
  } from "../stores.js";
  import { SETTINGS_ICON } from "../icons";

  import Picker from "./Picker.svelte";
  import Modal from "./Modal.svelte";

  let open = false;

  function parseSpeedInput(value: string): string {
    let number = parseInt(value);
    number = number < 1 ? 1 : number;
    return number.toString();
  }
</script>

<Modal width={300} headerText={"Options"} bind:open>
  <Picker
    store={onColour}
    inputType="color"
    name="on-colour"
    displayName="Screen On Colour"
    defaultValue={DEFAULT_ON_COLOUR}
    parser={null}
  />
  <Picker
    store={offColour}
    inputType="color"
    name="off-colour"
    displayName="Screen Off Colour"
    defaultValue={DEFAULT_OFF_COLOUR}
    parser={null}
  />
  <Picker
    store={gameSpeed}
    inputType="number"
    name="game-speed"
    displayName="Game Speed"
    defaultValue={DEFAULT_GAME_SPEED}
    parser={parseSpeedInput}
  />
</Modal>
<div class="settings" on:click={() => (open = true)}>
  {@html SETTINGS_ICON}
</div>

<style>
  .settings {
    position: fixed;
    right: -35px;
    top: -35px;
    z-index: 100;

    height: 70px;
    width: 70px;
    border-radius: 50%;

    box-shadow: rgb(0 0 0) -1px 1px 2px;
  }

  .settings:hover {
    right: 0px;
    top: 0px;

    opacity: 1;
  }
</style>

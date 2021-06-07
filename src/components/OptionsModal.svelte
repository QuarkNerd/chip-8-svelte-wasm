<script lang="typescript">
  // import ColourPicker from "./ColourPicker.svelte";
  import {
    offColour,
    onColour,
    gameSpeed,
    DEFAULT_OFF_COLOUR,
    DEFAULT_ON_COLOUR,
    DEFAULT_GAME_SPEED,
  } from "../stores.js";
  import Picker from "./Picker.svelte";
  import Modal from "./Modal.svelte";

  export let open: boolean;

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

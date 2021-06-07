<script lang="typescript">
  // import ColourPicker from "./ColourPicker.svelte";
  import {
    offColour,
    onColour,
    DEFAULT_OFF_COLOUR,
    DEFAULT_ON_COLOUR,
  } from "../stores.js";
  import Picker from "./Picker.svelte";
  import Modal from "./Modal.svelte";

  export let open: boolean;
</script>

<Modal width={300} headerText={"Options"} bind:open>
  <Picker
    store={onColour}
    inputType="color"
    name="on-colour"
    displayName="Screen On Colour"
    defaultValue={DEFAULT_ON_COLOUR}
  />
  <Picker
    store={offColour}
    inputType="color"
    name="off-colour"
    displayName="Screen Off Colour"
    defaultValue={DEFAULT_OFF_COLOUR}
  />
</Modal>

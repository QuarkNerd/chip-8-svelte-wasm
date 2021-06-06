<script lang="typescript">
  import ColourPicker from "./ColourPicker.svelte";
  import Modal from "./Modal.svelte";

  export let open: boolean;
</script>

<Modal width={300} headerText={"Options"} bind:open>
  <ColourPicker />
  <input type="number" />
</Modal>

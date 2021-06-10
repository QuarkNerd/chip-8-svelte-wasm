<script lang="ts">
  export let active: boolean;
</script>

<div on:click={() => (active = !active)}>
  <div class="holder" class:active />
  <div class="button" class:active />
</div>

<style>
  .holder {
    width: 30px;
    height: 10px;
    border-radius: 5px;

    background: linear-gradient(to right, maroon 50%, darkgray 50%);
    background-size: 200%;
    background-position: right bottom;
    box-shadow: rgb(0 0 0 / 80%) -1px 1px 2px inset;

    background-position: right;
    transition: 0.2s ease-out;
  }

  .holder.active {
    background-position: left;
  }

  .button {
    position: absolute;
    bottom: -2px;

    height: 14px;
    width: 14px;
    border-radius: 50%;

    background-color: rgb(78, 78, 78);

    background-position: right;
    transition: 0.2s ease-out;
  }

  .button.active {
    transform: translateX(16px);
  }
</style>

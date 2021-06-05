<script lang="typescript">
  import { fade, fly } from "svelte/transition";

  export let width: number;
  export let open: boolean;
  export let headerText: string;
</script>

{#if open}
  <div transition:fade class="overlay" class:open>
    <div
      transition:fly={{ y: -200, duration: 1000 }}
      class="modal"
      style="width:{width}px"
    >
      <div class="header">
        {headerText}
        <span on:click={() => (open = false)}>x</span>
        <hr />
      </div>
      <slot />
    </div>
  </div>
{/if}

<style>
  .header {
    padding-left: 5px;
  }

  .overlay {
    position: fixed;
    left: 0;
    top: 0;
    z-index: 271828;

    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.8);
  }

  .modal {
    margin: 15px auto;

    border-radius: 5px 0px 5px 5px;

    background-color: aliceblue;
    box-shadow: 2px 2px 8px rgba(0, 0, 0, 0.1);
  }

  span {
    float: right;
    padding: 0 8px;
  }

  span:hover {
    background-color: black;
    color: white;
  }

  hr {
    width: 80%;
  }
</style>

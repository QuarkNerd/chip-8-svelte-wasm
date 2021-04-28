<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  export let keysArray: Uint8Array;

  const keys = [
    { keyCode: 0x1, display: "1" },
    { keyCode: 0x2, display: "2" },
    { keyCode: 0x3, display: "3" },
    { keyCode: 0xc, display: "4" },
    { keyCode: 0x4, display: "Q" },
    { keyCode: 0x5, display: "W" },
    { keyCode: 0x6, display: "E" },
    { keyCode: 0xd, display: "R" },
    { keyCode: 0x7, display: "A" },
    { keyCode: 0x8, display: "S" },
    { keyCode: 0x9, display: "D" },
    { keyCode: 0xe, display: "F" },
    { keyCode: 0xa, display: "Z" },
    { keyCode: 0x0, display: "X" },
    { keyCode: 0xb, display: "C" },
    { keyCode: 0xf, display: "V" },
  ];

  onMount(() => {
    window.addEventListener("keypress", onKeyDown);
    window.addEventListener("keyup", onKeyUp);
  });

  onDestroy(() => {
    window.removeEventListener("keypress", onKeyDown);
    window.removeEventListener("keyup", onKeyUp);
  });

  function onKeyDown(ev: KeyboardEvent) {
    const display = ev.key.toUpperCase();
    const keyCode = getKeyCode(display);
    if (keyCode !== null) {
      if (!keysArray[keyCode]) {
        keysArray[keyCode] = 1;
      }
    }
  }

  function onKeyUp(ev: KeyboardEvent) {
    const display = ev.key.toUpperCase();
    const keyCode = getKeyCode(display);
    if (keyCode != null) keysArray[keyCode] = 0;
  }

  function getKeyCode(display: string): number | null {
    for (let i = 0; i < keys.length; i++) {
      if (keys[i].display === display) {
        return keys[i].keyCode;
      }
    }
    return null;
  }
</script>

<main>
  <div class="holder">
    {#each keys as { keyCode, display }}
      <div class="key" class:pressed={keysArray[keyCode]}>
        <p>
          {display}
        </p>
      </div>
    {/each}
  </div>
</main>

<style>
  .holder {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    grid-template-rows: 1fr 1fr 1fr 1fr;
    column-gap: 5%;
    row-gap: 5%;
    width: 40%;
    height: 20%;
  }

  .key {
    width: 100%;
    position: relative;
    background-color: #f4f4f4;
    border: 1px solid black;
    border-radius: 2px;
  }

  .key.pressed {
    background-color: lightgrey;
  }

  p {
    font-size: 2rem;
  }
</style>

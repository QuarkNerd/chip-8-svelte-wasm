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
        {display}
      </div>
    {/each}
  </div>
</main>

<style>
  .holder {
    height: 170px;
    width: 170px;
    margin: 15px auto;
    display: grid;
    grid-template-columns: 19% 19% 19% 19%;
    grid-template-rows: 19% 19% 19% 19%;
    column-gap: 8%;
    row-gap: 8%;
  }

  .key {
    border-radius: 50%;
    color: white;
    background-color: maroon;
    box-shadow: rgb(0 0 0) -2px 3px 5px,
      rgb(255 255 255 / 25%) -3px 4px 3px inset;
    text-align: center;
    line-height: 30px;
  }

  .key.pressed {
    box-shadow: -3px 4px 3px rgba(0, 0, 0, 0.25) inset,
      2px -2px 3px rgba(0, 0, 0, 0.25) inset;
    font-size: 0.9rem;
  }
</style>

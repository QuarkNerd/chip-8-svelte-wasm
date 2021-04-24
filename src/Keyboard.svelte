<script lang="ts">
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
</script>

<main>
  <div class="holder">
    {#each keys as {display}}
      <div class="key">
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
    column-gap: 2%;
    row-gap: 2%;
    width: 40%;
    height: 20%;
  }

  .key {
    width: 100%;
    position: relative;
    background-color: #f4f4f4;
    border:1px solid black;
    border-radius: 2px;
  }

  p {
    font-size: 2rem;
  }
</style>

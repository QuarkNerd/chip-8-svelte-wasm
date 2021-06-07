<script lang="typescript">
  export let store: SvelteStore<any>;
  export let inputType: string;
  export let name: string;
  export let displayName: string;
  export let defaultValue: any;

  function typeAction(node: HTMLInputElement) {
    node.type = inputType;
  }

  function reset() {
    const set = (store as any).set;
    if (set) set(defaultValue);
  }
</script>

<div>
  <div class="selector">
    <div class="label">
      <label for={`picker-${name}`}>{displayName}</label>
    </div>
    <div class="control">
      <input
        use:typeAction
        id={`picker-${name}`}
        name={`picker-${name}`}
        class:main-input={true}
        bind:value={$store}
      />
      <button name={`picker-${name}-reset`} on:click={reset}>Reset</button>
    </div>
  </div>
</div>

<style>
  .selector {
    margin: 5px;
    height: 30px;

    display: flex;
    justify-content: space-between;
  }

  .main-input {
    width: 50px;
    padding: 0;
    margin: 0;
    border: 0;
    height: 80%;
  }

  button {
    height: 100%
  }
</style>

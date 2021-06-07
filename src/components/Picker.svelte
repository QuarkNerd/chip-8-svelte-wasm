<script lang="typescript">
  import type { Writable } from "svelte/store";

  export let store: SvelteStore<any>;
  export let inputType: string;
  export let name: string;
  export let displayName: string;
  export let defaultValue: any;
  export let parser: ((input: string) => string) | null;

  let node: HTMLInputElement;

  function typeAction(_node: HTMLInputElement) {
    _node.type = inputType;
    node = _node;
  }

  function reset() {
    const set = (store as Writable<any>).set;
    if (set) set(defaultValue);
  }

  function onchange() {
    if (parser != null) {
      node.value = parser(node.value);
    }
  }
</script>

<div>
  <div class="selector">
    <div class="label">
      <label for={`picker-${name}`}>{displayName}</label>
    </div>
    <div class="control">
      <input
        on:change={onchange}
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
    height: 100%;
  }
</style>

<script lang="ts">
  import Emulator from "./Emulator.svelte";
  import GameHolder from "./GameHolder.svelte";
  import { quintOut } from 'svelte/easing';
	import { crossfade } from 'svelte/transition';
	import { flip } from 'svelte/animate';

		const bbb = crossfade({
		duration: d => Math.sqrt(d * 200),

		fallback(node, params) {
			const style = getComputedStyle(node);
			const transform = style.transform === 'none' ? '' : style.transform;

			return {
				duration: 600,
				easing: quintOut,
				css: t => `
					transform: ${transform} scale(${t});
					opacity: ${t}
				`
			};
		}
	});

  let games = [
    {active: false, game: "A"},
    {active: false, game: "B"},
    {active: false, game: "C"},
    ]
  setTimeout(() => console.log(games), 5000)
</script>

<main>
  <Emulator bind:games aaa={bbb}/>
  <GameHolder bind:games aaa={bbb}/>
</main>

<style>
  main {
    text-align: center;
  }
</style>

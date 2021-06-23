## Chip-8 Emulator

This project is an emulator for Chip-8 written in rust/wasm and svelte.
To view the project go [here](https://quarknerd.github.io/chip-8-svelte-wasm/)

Click on a game on the side to select it, press the toggle button underneath the game slot to pause the game. Use the settings in the top right corner to change screen colour
or emulation speed.

### Gotcha's

Here are some things that caught me out making this.

# Y-wrap

The spec claims that the screen wraps in X and Y. However not all games follow this.
So far I have discovered that Blitz requires Y-wrap to be turned off.

# Speed

Sometime the speed needs to be altered. For example, I can't get "Hidden" to function without slowing it down
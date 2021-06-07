import { writable } from "svelte/store";

export const DEFAULT_OFF_COLOUR = "#9aa040";
export const DEFAULT_ON_COLOUR = "#000000";

export const offColour = writable(DEFAULT_OFF_COLOUR);
export const onColour = writable(DEFAULT_ON_COLOUR);

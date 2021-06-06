import { writable } from "svelte/store";

const DEFAULT_OFF_COLOUR = "#9aa040";
const DEFAULT_ON_COLOUR = "#000000";

export const offColour = writable(DEFAULT_OFF_COLOUR);
export const onColour = writable(DEFAULT_ON_COLOUR);

export function resetOffColour() {
  offColour.set(DEFAULT_OFF_COLOUR);
}

export function resetOnColour() {
  onColour.set(DEFAULT_ON_COLOUR);
}

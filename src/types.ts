import { TransitionConfig, CrossfadeParams } from "svelte/transition";

type TransitionComponent = (
  node: Element,
  params: CrossfadeParams & {
    key: any;
  }
) => () => TransitionConfig;

export interface Transition {
  send: TransitionComponent;
  receive: TransitionComponent;
}

export interface Game {
  name: string;
  colour: string;
  yWrap: boolean;
}

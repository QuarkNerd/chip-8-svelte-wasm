<script lang="typescript">
  export let on: boolean;

  const frequency = 440;
  const audioCtx = new (window.AudioContext ||
    (window as any).webkitAudioContext)();
  const oscillator = audioCtx.createOscillator();
  const gainNode = audioCtx.createGain();

  gainNode.gain.value = 0.01;
  oscillator.type = "square";
  oscillator.frequency.value = frequency;
  oscillator.connect(gainNode);
  gainNode.connect(audioCtx.destination);
  oscillator.start();

  $: executeNotes(on);

  function executeNotes(on: boolean) {
    if (on) oscillator.connect(gainNode);
    else oscillator.disconnect(gainNode);
  }
</script>

<div class="speaker">
  <div class="band" />
  <div class="band" />
  <div class="band" />
  <div class="band" />
</div>

<style>
  .speaker {
    display: flex;
    flex-direction: row;
  }

  .band {
    margin-left: 8px;
    transform: rotate(40deg);
    height: 60px;
    width: 8px;
    border-radius: 8px;
    background: rgba(0, 0, 0, 0.35);
    box-shadow: rgb(0 0 0 / 60%) 3px 6px 1px inset;
  }
</style>

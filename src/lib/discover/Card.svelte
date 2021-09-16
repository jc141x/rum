<script>
  import Card from '$lib/Card.svelte';
  import { getBanner } from '$lib/util';

  export let game;
  export let selected = false;

  $: banner_src = getBanner(game);
</script>

<Card title={game.name} banner={banner_src} {selected} on:click />

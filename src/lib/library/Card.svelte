<script>
  import Card from '$lib/Card.svelte';

  export let game;
  export let selected;
</script>

<Card title={game.name} banner={game.banner} {selected} on:click />

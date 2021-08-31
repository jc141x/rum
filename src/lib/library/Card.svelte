<script>
  import Card from '$lib/Card.svelte';
  import banner from '$lib/assets/default_banner.png';

  export let game;
  export let selected;

  $: banner_src = game.banner === null ? banner : game.banner;
</script>

<Card title={game.name} banner={banner_src} {selected} on:click />

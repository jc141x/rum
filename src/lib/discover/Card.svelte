<script>
  import Card from '$lib/Card.svelte';

  export let game;
  export let selected = false;

  $: banner_src = `https://bkftwbhopivmrgzcagus.supabase.in/storage/v1/object/public/banners/${game.hash}.png`;
</script>

<Card title={game.name} banner={banner_src} {selected} on:click />

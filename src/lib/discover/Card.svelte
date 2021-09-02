<script>
  import Card from '$lib/Card.svelte';
  import banner from '$lib/assets/default_banner.png';

  export let game;
  export let selected = false;

  $: banner_src =
    game.banner_rel_path === null
      ? banner
      : `https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/${game.banner_rel_path}`;
</script>

<Card title={game.name} banner={banner_src} {selected} on:click />

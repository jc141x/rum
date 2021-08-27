<script>
  import { selectedGame } from '$lib/store.js';
  import command from '$lib/command';
  import Card from './Card.svelte';
  import banner from './default.png';
  import { Icon, Button } from 'svelte-materialify/src';
  import { mdiDownload } from '@mdi/js';

  export let game;

  $: banner_src =
    game.banner_path === null
      ? banner
      : `https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/${game.banner_path}`;

  $: subtitle = game.type.replace(
    /\w\S*/g,
    (txt) => txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase()
  );

  const handleMagnet = () => {
    command.misc('open_magnet', { game });
  };
</script>

<Card
  title={game.name}
  {subtitle}
  banner={banner_src}
  description={game.description}
  badges={game.genres}
  dangerBadges={game.nsfw ? ['18+'] : []}
>
  <Button icon slot="buttons" on:click={() => handleMagnet()}>
    <Icon path={mdiDownload} />
  </Button>
</Card>

<script>
  import banner from './default.png';
  import {
    Card,
    Button,
    Icon,
    CardText,
    CardSubtitle,
    CardTitle,
    Chip,
    CardActions
  } from 'svelte-materialify/src';
  import { mdiClose, mdiDownload } from '@mdi/js';
  import { selectedGame } from '$lib/store';
  import { createEventDispatcher } from 'svelte';
  import DownloadGameModal from './DownloadGameModal.svelte';
  const dispatch = createEventDispatcher();

  export let game;

  let download = false;

  $: banner_src =
    game.banner_path === null
      ? banner
      : `https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/${game.banner_path}`;

  $: subtitle = game.type.replace(
    /\w\S*/g,
    (txt) => txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase()
  );

  const handleDownload = () => {
    download = true;
  };
</script>

<div class="full">
  <Card class="pa-10" style="height: 100%; width: 100%; overflow-y: scroll">
    <Button on:click={() => selectedGame.set(null)} icon class="mb-10">
      <Icon path={mdiClose} />
    </Button>
    <img src={banner_src} alt="banner" />
    <CardTitle>{game.name}</CardTitle>
    <CardSubtitle>
      {subtitle}<br /><br />
      {#if game.nsfw}
        <Chip size="small" class="primary-color mr-2 mb-2">18+</Chip>
      {/if}
      {#each game.genres as genre}
        <Chip size="small" class="primary-color mr-2 mb-2">{genre}</Chip>
      {/each}
      <br />
    </CardSubtitle>

    <CardText>{game.description}</CardText>

    <CardActions>
      <Button icon on:click={() => handleDownload()}>
        <Icon path={mdiDownload} />
      </Button>
    </CardActions>
  </Card>

  <DownloadGameModal active={download} {game} on:close={() => (download = false)} />
</div>

<style>
  .full {
    height: 100%;
    width: 100%;
  }

  img {
    width: 100%;
  }
</style>

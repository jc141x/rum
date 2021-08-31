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
    CardActions,
    Menu,
    List,
    ListItem
  } from 'svelte-materialify/src';
  import { mdiClose, mdiDownload } from '@mdi/js';
  import { selectedLocalGame } from '$lib/store';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let game;

  $: banner_src = game.banner === null ? banner : game.banner;

  const handleLaunch = (script) => {
    command.library('run_game', { index: game.id, script });
  };
</script>

<div class="full">
  <Card class="pa-10" style="height: 100%; width: 100%; overflow-y: scroll">
    <Button on:click={() => selectedLocalGame.set(null)} icon class="mb-10">
      <Icon path={mdiClose} />
    </Button>
    <img src={banner_src} alt="banner" />
    <CardTitle>{game.name}</CardTitle>

    <CardSubtitle />
    <CardText>
      <b>Directory:</b>
      {game.executable_dir}
    </CardText>

    <CardActions>
      {#each game.scripts as script}
        <Button class="mr-5" on:click={() => handleLaunch(script.script)}>{script.name}</Button>
      {/each}
    </CardActions>
  </Card>
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

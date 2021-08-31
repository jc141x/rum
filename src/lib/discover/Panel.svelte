<script>
  import banner from '$lib/assets/default_banner.png';
  import { Button, Chip, Icon } from 'svelte-materialify/src';
  import Panel from '$lib/Panel.svelte';
  import DownloadGameModal from '$lib/discover/DownloadGameModal.svelte';
  import { mdiDownload } from '@mdi/js';

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

<Panel banner={banner_src} title={game.name} on:close>
  <div slot="subtitle">
    {subtitle}<br /><br />
    {#if game.nsfw}
      <Chip size="small" class="primary-color mr-2 mb-2">18+</Chip>
    {/if}
    {#each game.genres as genre}
      <Chip size="small" class="primary-color mr-2 mb-2">{genre}</Chip>
    {/each}
    <br />
  </div>
  <div slot="text">
    {game.description}
  </div>
  <div slot="actions">
    <Button icon on:click={() => handleDownload()}>
      <Icon path={mdiDownload} />
    </Button>
  </div>

  <DownloadGameModal slot="extra" active={download} {game} on:close={() => (download = false)} />
</Panel>

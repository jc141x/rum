<script>
  import banner from '$lib/assets/default_banner.png';
  import { Button, Chip, Icon } from 'svelte-materialify/src';
  import Panel from '$lib/Panel.svelte';
  import DownloadGameModal from '$lib/discover/DownloadGameModal.svelte';
  import { mdiDownload } from '@mdi/js';

  export let game;

  let download = false;

  $: banner_src =
    game.banner_rel_path === null
      ? banner
      : `https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/${game.banner_rel_path}`;

  const handleDownload = () => {
    download = true;
  };
</script>

<Panel banner={banner_src} title={game.name} on:close>
  <div slot="subtitle">
    {game.hash}<br /><br />
    {#each game.genres as genre}
      <Chip size="small" class="primary-color mr-2 mb-2">{genre}</Chip>
    {/each}
    <br />
    {#each game.tags as tag}
      <Chip size="small" class="primary-color mr-2 mb-2">{tag}</Chip>
    {/each}
    <br />
    {#each game.languages as language}
      <Chip size="small" class="primary-color mr-2 mb-2">{language}</Chip>
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

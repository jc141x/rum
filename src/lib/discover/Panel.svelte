<script>
  import banner from '$lib/assets/default_banner.png';
  import Icon from 'mdi-svelte';
  import Panel from '$lib/Panel.svelte';
  import DownloadGameModal from '$lib/discover/DownloadGameModal.svelte';
  import { mdiDownload } from '@mdi/js';

  export let game;

  let download = false;

  const handleDownload = () => {
    download = true;
    console.log('here');
  };

  $: banner_src = `https://bkftwbhopivmrgzcagus.supabase.in/storage/v1/object/public/banners/${game.hash}.png`;
</script>

<Panel banner={banner_src} title={game.name} on:close>
  <div slot="subtitle">
    {#if game.version}
      {game.version}
    {/if}
    <!--
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
-->
    <br />
  </div>
  <div slot="text" class="show-white-space">
    {game.description}
  </div>
  <div slot="actions">
    <button icon on:click={() => handleDownload()}>
      <Icon path={mdiDownload} />
    </button>
  </div>

  <DownloadGameModal slot="extra" active={download} {game} on:close={() => (download = false)} />
</Panel>

<style>
  .show-white-space {
    white-space: pre-wrap;
  }

  button {
    font-size: 20px;
  }
</style>

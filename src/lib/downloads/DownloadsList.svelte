<script>
  import command from '$lib/command';
  import DownloadItem from './DownloadItem.svelte';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let downloads = [];

  const handleButtonClick = async (torrent) => {
    switch (torrent.state) {
      case 'Paused':
        await command.download('resume', { client: torrent.client, torrentId: torrent.id });
        break;
      case 'Active':
        await command.download('pause', { client: torrent.client, torrentId: torrent.id });
        break;
    }

    dispatch('update');
  };
</script>

<div>
  {#each downloads as torrent}
    <div class="row">
      <DownloadItem {torrent} on:toggle-pause={() => handleButtonClick(torrent)} />
    </div>
  {/each}
</div>

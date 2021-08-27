<script>
  import command from '$lib/command';
  import { Row } from 'svelte-materialify/src';
  import DownloadItem from '$lib/DownloadItem.svelte';
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
    <Row class="mb-5">
      <DownloadItem {torrent} on:button-click={() => handleButtonClick(torrent)} />
    </Row>
  {/each}
</div>

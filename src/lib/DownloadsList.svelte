<script>
  import command from '$lib/command';
  import { Row } from 'svelte-materialify/src';
  import DownloadItem from '$lib/DownloadItem.svelte';

  export let downloads = [];
</script>

<div>
  {#each downloads as torrent}
    <Row class="mb-5">
      <DownloadItem {torrent} />
    </Row>
  {/each}
</div>

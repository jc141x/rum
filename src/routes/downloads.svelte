<script>
  import command from '$lib/command';
  import DownloadsList from '$lib/DownloadsList.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { Row, Col, Button, Icon } from 'svelte-materialify/src';
  import { mdiRefresh } from '@mdi/js';
  import DownloadItem from '$lib/DownloadItem.svelte';

  let downloads = [];
  let interval;

  const updateDownloads = async () => {
    downloads = await command.download('list_all_downloads');
  };

  onMount(async () => {
    await command.download('init_clients');
    await updateDownloads();
    interval = window.setInterval(updateDownloads, 1000);
  });

  onDestroy(() => {
    window.clearInterval(interval);
  });
</script>

<svelte:head>
  <title>Chad Launcher - Downloads</title>
</svelte:head>

<div class="ma-10">
  <!--
  <Row>
    <Col>
      <Button icon on:click={() => command.download('init_clients')}>
        <Icon path={mdiRefresh} />
      </Button>
    </Col>
  </Row>
    -->
  <DownloadsList {downloads} on:update={updateDownloads} />
</div>

<script>
  import command from '$lib/command';
  import DownloadsList from '$lib/DownloadsList.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { downloads } from '$lib/store';
  import { ProgressCircular } from 'svelte-materialify/src';
  import { mdiRefresh } from '@mdi/js';
  import DownloadItem from '$lib/DownloadItem.svelte';

  let interval;
  let list = [];

  downloads.subscribe(async ($downloads) => {
    list = await $downloads;
  });

  onMount(async () => {
    await command.download('init_clients');
    interval = window.setInterval(downloads.reload, 1000);
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
  <DownloadsList downloads={list} on:update={downloads.reload} />
</div>

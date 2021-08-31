<script>
  import command from '$lib/command';
  import DownloadsList from '$lib/downloads/DownloadsList.svelte';
  import { onMount, onDestroy } from 'svelte';
  import { downloads } from '$lib/store';

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

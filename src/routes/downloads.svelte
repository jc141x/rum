<script>
  import command from '$lib/command';
  import DownloadsList from '$lib/DownloadsList.svelte';

  let downloads = [];

  const updateDownloads = async () => {
    downloads = await command.download('list_all_downloads');
  };

  const initDownloads = async () => {
    await command.download('init_clients');
    await updateDownloads();
  };

  initDownloads();

  window.setInterval(updateDownloads, 1000);
</script>

<svelte:head>
  <title>Chad Launcher - Downloads</title>
</svelte:head>

<div class="ma-10">
  <DownloadsList {downloads} on:update={updateDownloads} />
</div>

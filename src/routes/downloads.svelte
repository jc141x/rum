<script>
  import command from '$lib/command';
  import DownloadsList from '$lib/DownloadsList.svelte';

  let downloads = [];

  const init_downloads = async () => {
    await command.download('init_clients');
    downloads = await command.download('list_all_downloads');
  };

  init_downloads();
</script>

<svelte:head>
  <title>Chad Launcher - Downloads</title>
</svelte:head>

<div class="ma-10">
  <DownloadsList {downloads} />
</div>

<script>
  import command from '$lib/command';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  let options = {
    host: 'http://localhost:8080',
    username: 'admin',
    password: ''
  };

  let errorMessage = null;

  export let clientName = '';

  const handleQBittorrentConnect = async () => {
    try {
      await command.download('add_qbittorrent_client', {
        name: clientName,
        options
      });
      errorMessage = null;
      dispatch('done');
    } catch (error) {
      errorMessage = `Failed to connect: ${JSON.stringify(error)}`;
    }
  };
</script>

<div>
  <h3>Configure qBittorrent</h3>
  <div class="options">
    <p>Name:</p>
    <input bind:value={clientName} />
    <p>Host:</p>
    <input bind:value={options.host} />
    <p>Username:</p>
    <input bind:value={options.username} />
    <p>Password:</p>
    <input type="password" bind:value={options.password} />
  </div>
  {#if errorMessage !== null}
    <div>
      {errorMessage}<br />
      <a href="/wiki#Chad-Launcher/User-Guide/Torrent-Clients">Need help?</a>
    </div>
  {/if}
  <div class="bottom">
    <button on:click={() => dispatch('close')}>Cancel</button>
    <button on:click={() => dispatch('back')}>Back</button>
    <button on:click={handleQBittorrentConnect}>Connect</button>
  </div>
</div>

<style>
  .bottom {
    position: absolute;
    bottom: 20px;
  }

  .options {
    display: grid;
    grid-template-columns: max-content auto;
    grid-gap: 10px;
    align-items: center;
    margin-bottom: 10px;
  }
</style>

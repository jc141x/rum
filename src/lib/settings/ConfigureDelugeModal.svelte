<script>
  import command from '$lib/command';
  import { fly } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  let hosts = [];
  export let clientName = 'Deluge';

  const options = {
    web_address: 'http://localhost:8112/json',
    web_password: 'deluge'
  };

  let errorMessage = null;

  let state = 'connect_web';
  let daemon = null;

  const handleDelugeConnectWeb = async () => {
    try {
      await command.download('create_deluge_client', {
        options
      });
      hosts = (await command.download('list_deluge_hosts')).map((host) => ({
        name: host.host,
        value: host.id
      }));
      errorMessage = null;
      state = 'connect_daemon';
    } catch (error) {
      errorMessage = `Failed to connect: ${JSON.stringify(error)}`;
    }
  };

  const handleDelugeConnectDaemon = async () => {
    try {
      await command.download('deluge_connect_daemon', {
        name: clientName,
        daemonId: daemon
      });
      errorMessage = null;
      dispatch('done');
    } catch (error) {
      state = 'connect_web';
      errorMessage = `Failed to connect: ${JSON.stringify(error)}`;
    }
  };
</script>

<div>
  <h3>Configure Deluge</h3>
  {#if state == 'connect_web'}
    <div in:fly={{ x: 100, duration: 200, delay: 200 }} out:fly={{ x: -100, duration: 200 }}>
      <h4>Connect to Web API</h4>
      <div class="options">
        <p>Name:</p>
        <input bind:value={clientName} />
        <p>Web address:</p>
        <input bind:value={options.web_address} />
        <p>Web password:</p>
        <input type="password" bind:value={options.web_address} />
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
        <button on:click={handleDelugeConnectWeb}>Connect</button>
      </div>
    </div>
  {:else if state == 'connect_daemon'}
    <div in:fly={{ x: 100, duration: 200, delay: 200 }} out:fly={{ x: -100, duration: 200 }}>
      <h4>Connect to daemon</h4>
      <div>
        <div class="options">
          <p>Daemon host:</p>
          <select bind:value={daemon}>
            {#each hosts as host}
              <option value={host.value}>{host.name}</option>
            {/each}
          </select>
        </div>
        {#if errorMessage !== null}
          <div>
            {errorMessage}<br />
            <a href="/wiki#Chad-Launcher/User-Guide/Torrent-Clients">Need help?</a>
          </div>
        {/if}
      </div>
      <div class="bottom">
        <button on:click={() => dispatch('close')}>Cancel</button>
        <button on:click={() => (state = 'connect_web')}>Back</button>
        <button on:click={handleDelugeConnectDaemon}>Connect</button>
      </div>
    </div>
  {/if}
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

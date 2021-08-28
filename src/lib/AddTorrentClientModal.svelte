<script>
  import {
    Overlay,
    Card,
    CardTitle,
    CardSubtitle,
    CardText,
    CardActions,
    Button,
    Select,
    TextField,
    Row
  } from 'svelte-materialify/src';
  import command from '$lib/command';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  const backends = ['Deluge', 'QBittorrent'];
  const defaultOptions = {
    Deluge: {
      web_address: 'http://localhost:8112/json',
      web_password: 'deluge'
    },
    QBittorrent: {
      host: 'http://localhost:8080',
      username: 'admin',
      password: ''
    }
  };

  let state = 'select_backend';
  let selectedBackend = null;
  let loading = false;
  let errorMessage = null;

  let clientName = '';
  let options = {};

  const handleSelectBackend = (backend) => {
    selectedBackend = backend;
    clientName = backend;
    options = defaultOptions[backend];
    errorMessage = null;
    state = 'configure_backend';
  };

  const handleQBittorrentConnect = async () => {
    loading = true;
    try {
      await command.download('add_qbittorrent_client', {
        name: clientName,
        options
      });
      errorMessage = null;
      state = 'done';
    } catch (error) {
      errorMessage = `Failed to connect: ${JSON.stringify(error)}`;
    }
    loading = false;
  };

  let delugeState = 'connect_web';
  let delugeDaemon = null;
  let delugeHosts = [];

  const handleDelugeConnectWeb = async () => {
    loading = true;
    try {
      await command.download('create_deluge_client', {
        options
      });
      delugeHosts = (await command.download('list_deluge_hosts')).map((host) => ({
        name: host.host,
        value: host.id
      }));
      errorMessage = null;
      delugeState = 'connect_daemon';
    } catch (error) {
      errorMessage = `Failed to connect: ${JSON.stringify(error)}`;
    }
    loading = false;
  };

  const handleDelugeConnectDaemon = async () => {
    loading = true;
    try {
      await command.download('deluge_connect_daemon', {
        name: clientName,
        daemonId: delugeDaemon
      });
      errorMessage = null;
      state = 'done';
    } catch (error) {
      delugeState = 'connect_web';
      errorMessage = `Failed to connect: ${JSON.stringify(error)}`;
    }
    loading = false;
  };
</script>

<Overlay active={true}>
  <Card>
    {#if state == 'select_backend'}
      <CardTitle>Choose a torrent client</CardTitle>
      <CardText class="ml-5">
        {#each backends as backend}
          <Row>
            <Button class="mb-5" on:click={() => handleSelectBackend(backend)}>{backend}</Button>
          </Row>
        {/each}
      </CardText>
      <CardActions>
        <Button class="mr-5" on:click={() => dispatch('close')}>Cancel</Button>
      </CardActions>
    {:else if state == 'configure_backend'}
      <CardTitle>Configure {selectedBackend}</CardTitle>
      {#if selectedBackend == 'QBittorrent'}
        <CardText class="ml-5">
          <Row class="mb-5">
            <TextField style="max-width: 77%" bind:value={clientName}>Name</TextField>
          </Row>
          <Row class="mb-5">
            <TextField style="max-width: 77%" bind:value={options.host}>Host</TextField>
          </Row>
          <Row class="mb-5">
            <TextField style="max-width: 77%" bind:value={options.username}>Username</TextField>
          </Row>
          <Row class="mb-5">
            <TextField password style="max-width: 77%" bind:value={options.password}>
              Password
            </TextField>
          </Row>
          {#if errorMessage !== null}
            <Row class="mb-5">
              {errorMessage}
            </Row>
          {/if}
        </CardText>
        <CardActions>
          <Button class="mr-5" on:click={() => dispatch('close')}>Cancel</Button>
          <Button class="mr-5" on:click={() => (state = 'select_backend')}>Back</Button>
          <Button on:click={handleQBittorrentConnect}>Connect</Button>
        </CardActions>
      {:else if selectedBackend == 'Deluge'}
        {#if delugeState == 'connect_web'}
          <CardText class="ml-5">
            <Row class="mb-5">
              <TextField style="max-width: 77%" bind:value={clientName}>Name</TextField>
            </Row>
            <Row class="mb-5">
              <TextField style="max-width: 77%" bind:value={options.web_address}>
                Web address
              </TextField>
            </Row>
            <Row class="mb-5">
              <TextField password style="max-width: 77%" bind:value={options.web_password}>
                Web password
              </TextField>
            </Row>
            {#if errorMessage !== null}
              <Row class="mb-5">
                {errorMessage}
              </Row>
            {/if}
          </CardText>
          <CardActions>
            <Button class="mr-5" on:click={() => dispatch('close')}>Cancel</Button>
            <Button class="mr-5" on:click={() => (state = 'select_backend')}>Back</Button>
            <Button on:click={handleDelugeConnectWeb}>Connect</Button>
          </CardActions>
        {:else if delugeState == 'connect_daemon'}
          <CardText class="ml-5">
            <Row>
              <Select items={delugeHosts} bind:value={delugeDaemon}>Host</Select>
            </Row>
            {#if errorMessage !== null}
              <Row class="mb-5">
                {errorMessage}
              </Row>
            {/if}
          </CardText>
          <CardActions>
            <Button class="mr-5" on:click={() => dispatch('close')}>Cancel</Button>
            <Button class="mr-5" on:click={() => (delugeState = 'connect_web')}>Back</Button>
            <Button on:click={handleDelugeConnectDaemon}>Connect</Button>
          </CardActions>
        {/if}
      {:else}
        Not implemented
      {/if}
    {:else if state == 'done'}
      <CardTitle>Client "{clientName}" added successfully.</CardTitle>
      <CardActions>
        <Button class="mr-5" on:click={() => dispatch('close')}>Close</Button>
      </CardActions>
    {/if}
  </Card>
</Overlay>

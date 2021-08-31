<script>
  import {
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

<Card>
  <CardTitle>Configure Deluge</CardTitle>
  {#if state == 'connect_web'}
    <CardSubtitle>Connect to Web API</CardSubtitle>
    <CardText class="ml-5">
      <Row class="mb-5">
        <TextField style="max-width: 77%" bind:value={clientName}>Name</TextField>
      </Row>
      <Row class="mb-5">
        <TextField style="max-width: 77%" bind:value={options.web_address}>Web address</TextField>
      </Row>
      <Row class="mb-5">
        <TextField type="password" style="max-width: 77%" bind:value={options.web_password}>
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
      <Button class="mr-5" on:click={() => dispatch('back')}>Back</Button>
      <Button on:click={handleDelugeConnectWeb}>Connect</Button>
    </CardActions>
  {:else if state == 'connect_daemon'}
    <CardSubtitle>Connect to daemon</CardSubtitle>
    <CardText class="ml-5">
      <Row>
        <Select items={hosts} bind:value={daemon}>Host</Select>
      </Row>
      {#if errorMessage !== null}
        <Row class="mb-5">
          {errorMessage}
        </Row>
      {/if}
    </CardText>
    <CardActions>
      <Button class="mr-5" on:click={() => dispatch('close')}>Cancel</Button>
      <Button class="mr-5" on:click={() => (state = 'connect_web')}>Back</Button>
      <Button on:click={handleDelugeConnectDaemon}>Connect</Button>
    </CardActions>
  {/if}
</Card>

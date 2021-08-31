<script>
  import {
    Card,
    CardTitle,
    CardText,
    CardActions,
    Button,
    TextField,
    Row
  } from 'svelte-materialify/src';
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

<Card>
  <CardTitle>Configure qBittorrent</CardTitle>
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
      <TextField type="password" style="max-width: 77%" bind:value={options.password}>
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
    <Button class="mr-5" on:click={() => dispatch('back')}>Back</Button>
    <Button on:click={handleQBittorrentConnect}>Connect</Button>
  </CardActions>
</Card>

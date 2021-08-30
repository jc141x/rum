<script>
  import {
    Overlay,
    Card,
    CardTitle,
    CardText,
    CardActions,
    Button,
    Select,
    TextField,
    Row,
    ProgressCircular
  } from 'svelte-materialify/src';
  import command from '$lib/command';
  import { torrentClients } from '$lib/store';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let game;

  let selectedClient = null;
  let savePath = '';
  let doneMessage = null;

  const handleDownload = () => {
    command
      .download('add_game', {
        client: selectedClient,
        game,
        options: {
          name: game.name,
          label: 'chad', // TODO Backend should only allow this label
          savePath
        }
      })
      .then(() => (doneMessage = 'Success!'))
      .catch((e) => (doneMessage = `Failed to add download: ${e}`));
  };
</script>

<Overlay active={true}>
  <Card>
    {#if doneMessage === null}
      {#await $torrentClients}
        <ProgressCircular indeterminate color="primary" />
      {:then clients}
        <CardTitle>Download {game.name}</CardTitle>
        <CardText class="ml-5">
          <Row class="mb-5">
            <Select items={clients} bind:value={selectedClient}>Torrent client</Select>
          </Row>
          <Row class="mb-5">
            <TextField style="max-width: 77%" bind:value={savePath}>Save path</TextField>
          </Row>
        </CardText>
        <CardActions>
          <Button class="mr-5" on:click={() => dispatch('close')}>Close</Button>
          <Button on:click={handleDownload}>Download</Button>
        </CardActions>
      {:catch error}
        <p style="color: red">{error.message}</p>
      {/await}
    {:else}
      <CardTitle>Download {game.name}</CardTitle>
      <CardText class="ml-5">
        {doneMessage}
      </CardText>
      <CardActions>
        <Button on:click={() => dispatch('close')}>Close</Button>
      </CardActions>
    {/if}
  </Card>
</Overlay>

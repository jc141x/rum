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

  export let game;

  let clients = [];
  let selectedClient = null;
  let savePath = '';
  let doneMessage = null;

  const load = async () => {
    clients = await command.download('list_clients');
  };

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

  $: {
    load();
  }
</script>

<Overlay active={true}>
  <Card>
    {#if doneMessage === null}
      <CardTitle>Download {game.name}</CardTitle>
      <CardText class="ml-5">
        <Row>
          <Select items={clients} bind:value={selectedClient}>Torrent client</Select>
        </Row>
        <Row>
          <TextField style="max-width: 77%" bind:value={savePath}>Save path</TextField>
        </Row>
      </CardText>
      <CardActions>
        <Button class="mr-5" on:click={() => dispatch('close')}>Close</Button>
        <Button on:click={handleDownload}>Download</Button>
      </CardActions>
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

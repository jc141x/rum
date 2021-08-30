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
  import { createEventDispatcher } from 'svelte';
  import ConfigureDelugeModal from './ConfigureDelugeModal.svelte';
  import ConfigureQBittorrentModal from './ConfigureQBittorrentModal.svelte';
  const dispatch = createEventDispatcher();

  const backends = ['Deluge', 'qBittorrent'];

  let state = 'select_backend';
  let selectedBackend = null;

  let clientName = '';

  const handleSelectBackend = (backend) => {
    selectedBackend = backend;
    clientName = selectedBackend;
    state = 'configure_backend';
  };
</script>

<Overlay active={true}>
  {#if state == 'select_backend'}
    <Card>
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
    </Card>
  {:else if state == 'configure_backend'}
    {#if selectedBackend == 'qBittorrent'}
      <ConfigureQBittorrentModal
        bind:clientName
        on:close
        on:done={() => (state = 'done')}
        on:back={() => (state = 'select_backend')}
      />
    {:else if selectedBackend == 'Deluge'}
      <ConfigureDelugeModal
        bind:clientName
        on:close
        on:done={() => (state = 'done')}
        on:back={() => (state = 'select_backend')}
      />
    {:else}
      Not implemented
    {/if}
  {:else if state == 'done'}
    <Card>
      <CardTitle>Client "{clientName}" added successfully.</CardTitle>
      <CardActions>
        <Button class="mr-5" on:click={() => dispatch('close')}>Close</Button>
      </CardActions>
    </Card>
  {/if}
</Overlay>

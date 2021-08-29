<script>
  import { config } from '$lib/store';
  import command from '$lib/command';
  import { Row, Col, TextField, Button, Divider } from 'svelte-materialify/src';
  import AddTorrentClientModal from '$lib/AddTorrentClientModal.svelte';

  let config_temp = {};
  config.subscribe(async (config) => {
    config_temp = JSON.parse(JSON.stringify(await config));
  });

  const addPath = () => (config_temp.library_paths = [...config_temp.library_paths, '']);
  const removePath = (path) =>
    (config_temp.library_paths = config_temp.library_paths.filter((p) => p != path));

  const save = async () => {
    await config.set(config_temp);
  };

  $: config_clients = config_temp?.torrent?.clients;
  let addClientModalActive = false;
  const addClient = () => {
    addClientModalActive = true;
  };

  const removeClient = async (name) => {
    await command.download('remove_client', { name });
    await config.load();
  };

  const handleModalClose = async () => {
    addClientModalActive = false;
    await config.load();
  };

  /*
  let active_clients = [];

  onMount(async () => {
    active_clients = await command.download('list_clients');
  });
*/
</script>

<svelte:head>
  <title>Chad Launcher - Settings</title>
</svelte:head>

<div class="ma-10">
  <Row noGutters class="mb-2">
    <h6>General options</h6>
  </Row>
  <Row>
    <Col sm={2}>Data path:</Col>
    <Col sm={10}>
      <TextField bind:value={config_temp.data_path} />
    </Col>
  </Row>
  <Row>
    <Col sm={2}>Terminal:</Col>
    <Col sm={10}>
      <TextField bind:value={config_temp.terminal} />
    </Col>
  </Row>
  <Row>
    <Divider class="mt-4 mb-4" />
  </Row>
  <Row noGutters class="mb-2">
    <h6>Library paths</h6>
  </Row>
  {#if config_temp.library_paths}
    {#each config_temp.library_paths as path}
      <Row noGutters>
        <Col>
          <TextField bind:value={path} />
        </Col>
        <Col sm={2} class="ml-5">
          <Button on:click={() => removePath(path)}>Remove</Button>
        </Col>
      </Row>
    {/each}
    <Row class="mt-2">
      <Col>
        <Button on:click={addPath}>Add path</Button>
      </Col>
    </Row>
  {/if}
  <Row>
    <Divider class="mt-4 mb-4" />
  </Row>
  <Row noGutters class="mb-2">
    <h6>Torrent Clients</h6>
  </Row>
  {#if config_clients}
    {#each Object.entries(config_clients) as [name, config]}
      <Row noGutters class="mb-2">
        <Col>
          <h8>{name}</h8>
        </Col>
        <Col sm={2}>
          <Button on:click={() => removeClient(name)}>Remove</Button>
        </Col>
      </Row>
    {/each}
  {/if}
  <Row noGutters>
    <Col>
      <Button on:click={addClient}>Add client</Button>
    </Col>
  </Row>
  <Row>
    <Divider class="mt-4" />
  </Row>
  <Row>
    <Col>
      <Button on:click={save}>Save</Button>
    </Col>
  </Row>
  {#if addClientModalActive}
    <AddTorrentClientModal on:close={handleModalClose} />
  {/if}
</div>

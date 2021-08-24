<script>
  import { config } from '$lib/store';
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import {
    Row,
    Col,
    TextField,
    Button,
    Divider,
    Overlay,
    Card,
    CardTitle,
    CardActions
  } from 'svelte-materialify/src';

  let modal_active = false;
  let config_temp = {};

  const loadConfig = async () => {
    config_temp = await invoke('get_config')
    $config = config_temp
  };
  loadConfig().then(() => console.log(config_temp));

  const addPath = () => (config_temp.library_paths = [...config_temp.library_paths, '']);

  const save = async () => {
    await invoke('set_config', { newConfig: config_temp });
    await invoke('save_config');
    await loadConfig();
    modal_active = true;
  };

  $: console.log($config);
</script>

<svelte:head>
  <title>Chad Launcher - Settings</title>
</svelte:head>

<div>
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
        <TextField bind:value={path} />
      </Row>
    {/each}
    <Row class="mt-2">
      <Col>
        <Button on:click={addPath}>Add path</Button>
      </Col>
    </Row>
  {/if}
  <Row>
    <Divider class="mt-4" />
  </Row>
  <Row>
    <Col>
      <Button on:click={save}>Save</Button>
    </Col>
  </Row>

  <Overlay opacity={0.5} active={modal_active}>
    <Card>
      <CardTitle>Settings saved!</CardTitle>
      <CardActions>
        <Button
          on:click={() => {
            modal_active = false;
          }}
        >
          Ok
        </Button>
      </CardActions>
    </Card>
  </Overlay>
</div>

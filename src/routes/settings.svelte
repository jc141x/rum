<script>
  import { config } from '$lib/store';
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import { Row, Col, TextField, Button, Divider } from 'svelte-materialify/src';

  let config_temp = {};

  const loadConfig = async () => {
    config_temp = await invoke('get_config');
    $config = config_temp;
  };
  loadConfig().then(() => console.log(config_temp));

  const addPath = () => (config_temp.library_paths = [...config_temp.library_paths, '']);
  const removePath = (path) =>
    (config_temp.library_paths = config_temp.library_paths.filter((p) => p != path));

  const save = async () => {
    await invoke('set_config', { newConfig: config_temp });
    await invoke('save_config');
    await loadConfig();
  };

  $: console.log($config);
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
        <Col>
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
    <Divider class="mt-4" />
  </Row>
  <Row>
    <Col>
      <Button on:click={save}>Save</Button>
    </Col>
  </Row>
</div>

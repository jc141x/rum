<script>
  import { config } from '$lib/store';
  import command from '$lib/command';
  import { styles, defaultStyles } from '$lib/styles';
  import Icon from 'mdi-svelte';
  import { mdiDelete, mdiFolder, mdiReload, mdiUndo } from '@mdi/js';
  import { open } from '../../node_modules/@tauri-apps/api/dialog';
  import { decorations } from '$lib/store';
  import ColorSetting from '$lib/ColorSetting.svelte';
  import AddTorrentClientModal from '$lib/settings/AddTorrentClientModal.svelte';

  let config_temp = {};

  config.subscribe(async (config) => {
    config_temp = JSON.parse(JSON.stringify(await config));
  });

  const addPath = () => (config_temp.library_paths = [...config_temp.library_paths, '']);
  const removePath = (path) =>
    (config_temp.library_paths = config_temp.library_paths.filter((p) => p != path));
  const selectPath = async (i) => {
    config_temp.library_paths[i] = await open({
      defaultPath: config_temp.library_paths[i],
      directory: true
    });
  };

  const selectDataPath = async () => {
    config_temp.data_path = await open({
      defaultPath: config_temp.data_path,
      directory: true
    });
  };

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
    config.reload();
  };

  const handleModalClose = async () => {
    addClientModalActive = false;
    config.reload();
  };

  const undoColor = (key) => {
    $styles[key] = defaultStyles[key];
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

<div class="top">
  <div class="row">
    <h6>General options</h6>
  </div>
  <div class="row">
    <div class="settings-grid">
      <div>Data path:</div>
      <div>
        <input bind:value={config_temp.data_path} />
      </div>
      <div>
        <button on:click={selectDataPath}><Icon path={mdiFolder} /></button>
      </div>
      <div>Terminal:</div>
      <div>
        <input bind:value={config_temp.terminal} />
      </div>
    </div>
  </div>
  <div class="row">
    <hr class="divider" />
  </div>
  <div class="row">
    <h6>Library paths</h6>
  </div>
  {#if config_temp.library_paths}
    {#each config_temp.library_paths as path, i}
      <div class="row">
        <div>
          <input bind:value={path} />
        </div>
        <div>
          <button on:click={() => selectPath(i)}><Icon path={mdiFolder} /></button>
        </div>
        <div>
          <button on:click={() => removePath(path)}><Icon path={mdiDelete} /></button>
        </div>
      </div>
    {/each}
    <div class="row">
      <div>
        <button on:click={addPath}>Add path</button>
      </div>
    </div>
  {/if}
  <div class="row">
    <hr class="divider" />
  </div>
  <div class="row">
    <h6>Torrent Clients</h6>
  </div>
  <div class="row">
    <div class="clients-grid">
      {#if config_clients}
        {#each Object.entries(config_clients) as [name, config]}
          <div>
            <h8>{name}</h8>
          </div>
          <div>
            <button on:click={() => removeClient(name)}><Icon path={mdiDelete} /></button>
          </div>
        {/each}
      {/if}
    </div>
  </div>
  <div class="row">
    <div>
      <button on:click={addClient}>Add client</button>
    </div>
  </div>
  <div class="row">
    <hr class="divider" />
  </div>
  <div class="row">
    <h6>Theme</h6>
  </div>
  <div class="row">
    <div class="settings-grid">
      <div>Window decorations</div>
      <select bind:value={$decorations}>
        <option value="system">System</option>
        <option value="disabled">Disabled</option>
        <option value="left">Left</option>
        <option value="right">Right</option>
      </select>
      <div />
      {#each Object.entries($styles) as [key, value]}
        <div>{key}:</div>
        <div>
          <ColorSetting {key} />
        </div>
        <button on:click={() => undoColor(key)}><Icon path={mdiUndo} /></button>
      {/each}
    </div>
  </div>
  <div class="row">
    <hr class="divider" />
  </div>
  <div class="row">
    <div>
      <button on:click={save}>Save</button>
    </div>
  </div>
  {#if addClientModalActive}
    <AddTorrentClientModal on:close={handleModalClose} active={addClientModalActive} />
  {/if}
</div>

<style>
  .top {
    margin: 10px;
    margin-top: 20px;
    overflow-y: auto;
    height: calc(100vh - 110px);
  }

  .row {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin: 10px;
  }

  .row > * {
    margin-right: 10px;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: max-content auto 50px;
    grid-gap: 20px;
    align-items: center;
  }

  .clients-grid {
    display: grid;
    grid-template-columns: max-content 50px;
    grid-gap: 20px;
    align-items: center;
  }

  input {
    width: 500px;
  }
</style>

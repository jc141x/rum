<script>
  import { config } from '$lib/store';
  import command from '$lib/command';
  import { styles, defaultStyles } from '$lib/styles';
  import Icon from 'mdi-svelte';
  import { mdiDelete, mdiFolder, mdiReload, mdiUndo } from '@mdi/js';
  import { open } from '../../node_modules/@tauri-apps/api/dialog';
  import { decorations } from '$lib/store';
  import ColorSetting from '$lib/ColorSetting.svelte';

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


  const handleModalClose = async () => {
    addClientModalActive = false;
    config.reload();
  };

  const undoColor = (key) => {
    $styles[key] = defaultStyles[key];
  };

</script>

<svelte:head>
  <title>Chad Launcher - Settings</title>
</svelte:head>

<section class="settings-group">
      <h6>General options</h6>
      <div class="input-wrapper">
        <label for="data-path">Data path:</label>
        <div class="input-row">
          <input id="data-path" bind:value={config_temp.data_path} />
          <button on:click={selectDataPath}><Icon path={mdiFolder} /></button>
        </div>
      </div>
      <div class="input-wrapper">
        <label for="terminal">Terminal:</label>
        <div class="input-row">
          <input id="terminal" bind:value={config_temp.terminal} />
        </div>
      </div>
    </section>
    <secion class="settings-group">
      <h6>Library paths</h6>
      <a href="/wiki#Chad-Launcher/User-Guide/Game-Library">What is this?</a>
      {#if config_temp.library_paths}
      {#each config_temp.library_paths as path, i}
              <div class="input-wrapper">
                <div class="input-row">
                  <input bind:value={path} />
                  <button on:click={() => selectPath(i)}><Icon path={mdiFolder} /></button>
                  <button on:click={() => removePath(path)}><Icon path={mdiDelete} /></button>
                </div>
              </div>
          {/each}
          <div class="input-wrapper">
            <div class="input-row">
              <button on:click={addPath}>Add path</button>
            </div>
          </div>
        {/if}
    </secion>
    <section class="settings-group">
      <h6>Theme</h6>
      <div class="input-wrapper">
        <label for="window-decorations">Window decorations</label>
        <div class="input-row">
          <select id="window-decorations" bind:value={$decorations}>
            <option value="system">System</option>
            <option value="disabled">Disabled</option>
            <option value="left">Left</option>
            <option value="right">Right</option>
          </select>
        </div>
      </div>
      <div />
      {#each Object.entries($styles) as [key, value]}
      <div class="input-wrapper">
        <label for="color-{key}">{key}:</label>
        <div class="input-row">
          <div>
            <ColorSetting id="color-{key}" {key} />
          </div>
          <button on:click={() => undoColor(key)}><Icon path={mdiUndo} /></button>
        </div>
      </div>
      {/each}
    </section>
      <button on:click={save}>Save</button>

<style>
  .settings-group {
    display: flex;
    width: 600px;
    padding: 1rem;
    flex-direction: column;
    margin-top:2rem;
    margin-bottom: 1rem;
    border: 2px solid var(--primary);
    border-radius: 10px;
  }
  .settings-group h6 {
    margin-top: -2rem;    
  }
  .input-wrapper {
    display: flex;
    flex-direction: column;
  }
  .input-row {
    display: flex;
    flex-direction: row;
    align-content: stretch;
    margin-bottom:1rem;
  }
  .input-row input,div,select{
    flex-grow: 2;
  }
  .input-row button {
    flex-grow: 1;
  }
  select {
    appearance: menulist;
    background-color: var(--secondary);
  }
</style>

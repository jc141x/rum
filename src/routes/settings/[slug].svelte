<script>
  import command from '$lib/command';
  import { slide } from 'svelte/transition'
  import { styles, defaultStyles } from '$lib/styles';
  import Icon from 'mdi-svelte';
  import { mdiDelete, mdiFolder, mdiContentSave, mdiUndo } from '@mdi/js';
  import { open } from '../../../node_modules/@tauri-apps/api/dialog';
  import { cardTextSize, config, decorations, cardWidth, cardHeight, allfiles } from '$lib/store';
  import ColorSetting from '$lib/ColorSetting.svelte';
  import { page } from '$app/stores';
  const slug = $page.params.slug;
  const settingPages = ['general', 'library', 'theme', 'other'];
  let config_temp = {};

  config.subscribe(async (config) => {
    config_temp = JSON.parse(JSON.stringify(await config));
  });

  const addPath = () => (config_temp.library_paths = [...config_temp.library_paths, '']);
  const removePath = (path) =>
    (config_temp.library_paths = config_temp.library_paths.filter((p) => p != path));
  const selectPath = async (i) => {
    config_temp.library_paths[i] = await open({
      defaultPath: config_temp.library_paths[i] ? config_temp.library_paths[i] : '/',
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
  <title>Rum - Settings</title>
</svelte:head>
<main>
<aside>
  <nav>
    <ul>
      {#each settingPages as page}
        {#if page == slug}
          <li class="active">
            <a href={`/settings/${page}`}>{page}</a>
          </li>
        {:else}
          <li>
            <a href={`/settings/${page}`}>{page}</a>
          </li>
        {/if}
      {/each}
    </ul>
    <button on:click={save}><Icon path={mdiContentSave} /></button>
  </nav>
</aside>
<section in:slide>
{#if slug == 'general'}
<article class="settings-group">
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
</article>
{/if}
{#if slug == 'library'}
<article class="settings-group">
  <div>
    <h6 class="inline-block">Library paths</h6>
    <a class="what" href="/wiki#user-guide/game-library.md">What is this?</a>
  </div>
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
</article>
{/if}
{#if slug == 'theme'}
<article class="settings-group">
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
  <div class="input-wrapper">
    <label for="card-width">Card width:</label>
    <div class="input-row">
      <input type="range" min="100" max="400" id="card-width" bind:value={$cardWidth} />
    </div>
    <small>{$cardWidth}px</small>
  </div>
  <div class="input-wrapper">
    <label for="window-decorations">Card aspect ratio:</label>
    <div class="input-row">
      <select id="window-decorations" bind:value={$cardHeight}>
        <option value="1/1">1:1</option>
        <option value="3/2">2:3</option>
        <option value="31/22">22:31</option>
        <option value="43/92">92:43</option>
      </select>
    </div>
  </div>
  <div />
  <div class="input-wrapper">
    <label for="card-width">Card text size:</label>
    <div class="input-row">
      <input type="range" min="50" max="150" id="card-width" bind:value={$cardTextSize} />
    </div>
    <small>{$cardTextSize}%</small>
  </div>
</article>
{/if}
{#if slug == 'other'}
<article class="settings-group">
  <h6>Other</h6>
  <p>show all files in banner picker</p>
  <input bind:checked={$allfiles} type="checkbox" name="allfiles" />
</article>
{/if}
</section>
</main>
<style>
  main {
    display: grid;
    grid-template-columns: min-content 1fr;
    height: 100%;
  }
  aside {
    grid-column: 1 / 2;
    padding-inline: 0 1rem;
    border-right: 1px solid var(--primary);
  }
  aside nav ul {
    list-style: none;
    padding: 0;
    }
  aside nav li a {
    display: block;
    padding-inline: 3em;
    text-decoration: none;
    border: 1px solid var(--secondary);
    border-radius: 0.25em;
    margin-block: 0.5rem;
    text-align: center;
    text-transform: capitalize;
    }
  aside nav li.active a,
  aside nav li a:hover { 
    border-color: var(--primary)
  }
  aside nav button {
    display: block;
    margin-block: 3rem;
    width: 100%;
  }
  section {
    padding: 1rem;
  }
  /*
  TODO: clean up this ↓ shit 
  */
  [type='range'] {
    appearance: slider-horizontal;
  }
  label {
    font-size: 85%;
    line-height: 85%;
  }
  .settings-group {
    display: flex;
    min-width: 600px;
    max-width: 50%;
    flex-direction: column;
  }
  .settings-group h6 {
    margin-bottom: 1.5rem;
    display: inline-block;
  }
  .input-wrapper {
    display: flex;
    flex-direction: column;
    margin: 1rem 0;
  }
  .input-row {
    display: flex;
    flex-direction: row;
    align-content: stretch;
  }
  .input-row input,
  div,
  select {
    flex-grow: 2;
  }
  .input-row button {
    flex-grow: 1;
  }
  .what {
    float: right;
    font-size: smaller;
  }
  [type="checkbox"] {
    width: 1rem;
    height: 1rem;
  }
</style>

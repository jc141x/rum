<script>
  import { slide } from 'svelte/transition'
  import { styles, defaultStyles } from '$lib/styles';
  import IconContentSave from '~icons/mdi/content-save';
  import IconTrash from '~icons/mdi/trash';
  import IconFolder from '~icons/mdi/folder';
  import { open } from '../../../node_modules/@tauri-apps/api/dialog';
  import { config, decorations, allfiles } from '$lib/store';
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
          <li class="tabs">
            <a class="active" href={`/settings/${page}`}>{page}</a>
          </li>
        {:else}
          <li class="tabs">
            <a href={`/settings/${page}`}>{page}</a>
          </li>
        {/if}
      {/each}
    </ul>
    <button class="button icon-only" on:click={save}><IconContentSave /></button>
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
      <button on:click={selectDataPath}><IconFolder /></button>
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
    <a class="is-right" href="/wiki#user-guide/game-library.md">What is this?</a>
  </div>
  {#if config_temp.library_paths}
    {#each config_temp.library_paths as path, i}
      <div class="input-wrapper">
        <div class="input-row">
          <input bind:value={path} />
          <button on:click={() => selectPath(i)}><IconFolder /></button>
          <button on:click={() => removePath(path)}><IconTrash /></button>
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
{#if slug == 'theme'}  import command from '$lib/command';
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
  }
  nav {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  nav ul {
    display: grid;
    grid-template-columns: 1fr;
    grid-gap: 1rem;
    list-style-type: none;
    padding: 0;
  }
  nav ul li {
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
  }
</style>
<script>
  import { slide } from 'svelte/transition'
  import { styles, defaultStyles } from '$lib/styles';
  import IconContentSave from '~icons/mdi/content-save';
  import IconTrash from '~icons/mdi/trash';
  import IconFolder from '~icons/mdi/folder';
  import { open } from '../../../node_modules/@tauri-apps/api/dialog';
  import { config, decorations, allfiles } from '$lib/store';
  import { page } from '$app/stores';
import command from '$lib/command';
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

  const handleDeleteAll = async () => {
    command.library('delete_all_banners').catch(console.error);
  }
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
  <table>
    <tr>
      <td>
        <label for="data-path">Data path:</label>
      </td>
      <td>
        <input id="data-path" bind:value={config_temp.data_path} />
      </td>
      <td>
        <button on:click={selectDataPath}><IconFolder /></button>
      </td>
    </tr>
    <tr>
      <td>
        <label for="terminal">Terminal:</label>
      </td>
      <td>
        <input id="terminal" bind:value={config_temp.terminal} />
      </td>
    </tr>
  </table>
</article>
{/if}
{#if slug == 'library'}
<article class="settings-group">
  <h6 class="inline-block">Library</h6>
  <a class="is-right" href="/wiki#user-guide/game-library.md">What is this?</a>
  <table>
    {#if config_temp.library_paths}
    {#each config_temp.library_paths as path, i}
    <tr>
      <td>
          <input bind:value={path} />
      </td>
      <td>
        <button on:click={() => selectPath(i)}><IconFolder /></button>
      </td>
      <td>
          <button on:click={() => removePath(path)}><IconTrash /></button>
      </td>
    </tr>
    {/each}
    {/if}
    <button on:click={addPath}>Add path</button>
  </table>
</article>
{/if}
{#if slug == 'theme'}
<article class="settings-group">
  <h6>Theme</h6>
  <table>
    <tr>
      <td>
        <label for="window-decorations">Window decorations</label>
      </td>
      <td>
        <select id="window-decorations" bind:value={$decorations}>
          <option value="system">System</option>
          <option value="disabled">Disabled</option>
          <option value="left">Left</option>
          <option value="right">Right</option>
        </select>
      </td>
    </tr>
  </table>
  <div />
</article>
{/if}
{#if slug == 'other'}
<article class="settings-group">
  <h6>Other</h6>
  <table>
    <tr>
      <td>
        <label for="allfiles">show all files in banner picker</label>
      </td>
      <td>
        <input bind:checked={$allfiles} type="checkbox" name="allfiles" id="allfiles"/>
      </td>
    </tr>
    <tr>
      <td>
        Delete all banners
      </td>
      <td>
        <button class="button error" on:click={handleDeleteAll}>Delete</button>
      </td>
    </tr>
  </table>
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
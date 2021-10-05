<script>
  import banner from '$lib/assets/default_banner.png';
  import Panel from '$lib/Panel.svelte';
  import command from '$lib/command';
  import Icon from 'mdi-svelte';
  import { mdiLoading, mdiFolder, mdiPlay } from '@mdi/js';

  export let game;
  let loading = false;

  $: banner_src = game.banner === null ? banner : game.banner;

  const handleLaunch = async (script) => {
    loading = script;
    await command.library('run_game', { index: game.id, script });
    loading = false;
  };
  const handlePath = () => {
    command.library('open_folder', { index: game.id });
  };
</script>

<Panel banner={banner_src} title={game.name} on:close>
  <div slot="text">
    <b>Directory:</b>
    {game.executable_dir}
  </div>
  <div class="actions" slot="actions">
    <button on:click={handlePath}><Icon path={mdiFolder} /></button>
    {#each game.scripts as script}
      <button on:click={() => handleLaunch(script.script)}>
        <span class="script--button">
          {#if loading == script.script}
            <Icon path={mdiLoading} spin />
          {:else}
            <Icon path={mdiPlay} />
          {/if}
          {script.name}
        </span>
      </button>
    {/each}
  </div>
</Panel>

<style>
  button {
    font-size: 20px;
  }
  button span {
    vertical-align: bottom;
  }
  .actions {
    display: flex;
    flex: row;
  }
  .script--button {
    display: flex;
    gap: 5px;
    flex: row;
  }
</style>

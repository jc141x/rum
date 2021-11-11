<script>
  import Panel from '$lib/Panel.svelte';
  import command from '$lib/command';
  import Icon from 'mdi-svelte';
  import { mdiLoading, mdiFolder, mdiPlay, mdiCog } from '@mdi/js';
  import GameSettings from '$lib/library/GameSettings.svelte';

  export let game;
  let loading = false;
  let SettingsActive = false;

  const handleLaunch = async (script) => {
    loading = script;
    await command.library('run_game', { index: game.id, script });
    loading = false;
  };
  const handlePath = () => {
    command.library('open_folder', { index: game.id });
  };

  const handleOpenSettings = () => {
    SettingsActive = true;
  };
  const handleCloseSettings = () => {
    SettingsActive = false;
  };
</script>

{#if SettingsActive}
  <GameSettings on:close={handleCloseSettings} {game} />
{/if}
<Panel title={game.name} on:close>
  <div slot="text">
    <b>Directory:</b>
    {game.executable_dir}
  </div>
  <div class="actions" slot="actions">
    <button on:click={handleOpenSettings}
      ><span class="align-fix"><Icon path={mdiCog} /></span></button
    >
    <button on:click={handlePath}><span class="align-fix"><Icon path={mdiFolder} /></span></button>
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
  .actions {
    display: flex;
    flex: row;
  }
  .script--button {
    display: flex;
  }
  .align-fix {
    height: 1.5rem;
    margin-top: auto;
    display: flex;
    align-self: center;
    justify-content: stretch;
  }
</style>

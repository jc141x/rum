<script>
  import command from '$lib/command';

  import IconLoading from '~icons/mdi/loading';
  import IconFolder from '~icons/mdi/folder';
  import IconPlay from '~icons/mdi/play';
  import IconCog from '~icons/mdi/cog';
  import IconWindows95 from '~icons/simple-icons/windows95';
  import IconGnu from '~icons/simple-icons/gnu';
  import IconLinux from '~icons/simple-icons/linux';

  import GameSettings from '$lib/library/GameSettings.svelte';
import { onMount } from 'svelte';

  export let game;
  let running = false;
  let SettingsActive = false;
  const handleLaunch = async (script) => {
    running = script;
    await command.library('run_game', { index: game.id, script });
    document.activeElement.blur();
    running = false;
  };
  const handlePath = () => {
    command.library('open_folder', { index: game.id });
    document.activeElement.blur();
  };

  const handleOpenSettings = () => {
    SettingsActive = true;
  };
  const handleCloseSettings = () => {
    SettingsActive = false;
  };
  const wine = game.scripts.find(o => o.platform == 'Wine') ? true : false;
  const native = game.scripts.find(o => o.platform == 'Native') ? true : false;
  let hero = "";
  onMount(async () => {
    try {
      hero =  await command.library('sgdb_hero_fetch', { index: game.id} );
    } catch (e) {
      console.error(e);
    }
  });
</script>

{#if SettingsActive}
  <GameSettings on:close={handleCloseSettings} {game} />
{/if}
<div class="panel col-12" style="background-image: linear-gradient(90deg, black 10%, transparent), url('{hero}')">
  <div class="title">
    <h2>{game.name}&#9;<span>        {#if wine}
      <IconWindows95 style="height: 0.75em"/>
    {/if}
    {#if native}
      <IconGnu style="height: 0.75em"/><IconLinux style="height: 0.75em"/>
    {/if}</span></h2>
  </div>
  <div class="actions">
    <button class="button primary outline icon-only" on:click={handleOpenSettings}
    ><IconCog/></button
  >
  <button class="button primary outline icon-only" on:click={handlePath}><IconFolder /></button>
  {#each game.scripts as script}
    <button class="button primary outline" on:click={() => handleLaunch(script.script)}>
        {#if running == script.script}
          <IconLoading/>
        {:else}
          <IconPlay />
        {/if}
        {script.name}
    </button>
  {/each}
  </div>
</div>

<style>
  .panel {
    background-position: right;
    background-size: 100% auto;
    background-repeat: no-repeat;
    background-color: black;
    padding-block: 0;
    height: 200px;
  }

  .panel:not(:hover) .actions {
    display: none;
  }
</style>
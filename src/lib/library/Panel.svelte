<script>
  import banner from '$lib/assets/default_banner.png';
  import Panel from '$lib/Panel.svelte';
  import command from '$lib/command';
  import Icon from 'mdi-svelte';
  import { mdiFolder, mdiPlay } from '@mdi/js';

  export let game;

  $: banner_src = game.banner === null ? banner : game.banner;

  const handleLaunch = (script) => {
    command.library('run_game', { index: game.id, script });
  };
  const handlePath = () => {
    command.library('open_folder', {index: game.id});
  }
</script>

<Panel banner={banner_src} title={game.name} on:close>
  <div slot="text">
    <b>Directory:</b>
    {game.executable_dir}
  </div>
  <div slot="actions">
    <button on:click={handlePath}><Icon path={mdiFolder} /></button>
    {#each game.scripts as script}
      <button on:click={() => handleLaunch(script.script)}><span><Icon path={mdiPlay}/>{script.name}</span></button>
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
</style>

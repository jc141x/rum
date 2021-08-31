<script>
  import banner from '$lib/assets/default_banner.png';
  import { Button } from 'svelte-materialify/src';
  import Panel from '$lib/Panel.svelte';

  export let game;

  $: banner_src = game.banner === null ? banner : game.banner;

  const handleLaunch = (script) => {
    command.library('run_game', { index: game.id, script });
  };
</script>

<Panel banner={banner_src} title={game.name} on:close>
  <div slot="text">
    <b>Directory:</b>
    {game.executable_dir}
  </div>
  <div slot="actions">
    {#each game.scripts as script}
      <Button class="mr-5" on:click={() => handleLaunch(script.script)}>{script.name}</Button>
    {/each}
  </div>
</Panel>

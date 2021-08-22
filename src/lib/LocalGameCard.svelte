<script>
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import Card from './Card.svelte';
  import banner from './default.png';

  export let game;

  $: banner_src = game.banner === null ? banner : game.banner;

  const handleLaunch = (script) => {
    invoke('run_game', { index: game.id, script });
  };

</script>

<Card title={game.name} banner={banner_src} height={150}>
    <!-- Does not work without halfmoon.js
  <div class="dropdown" slot="buttons">
    <button
      class="btn"
      data-toggle="dropdown"
      type="button"
      id="dropdown-toggle-btn-1"
      aria-haspopup="true"
      aria-expanded="false"
    >
      Launch <i class="fa fa-angle-down ml-5" aria-hidden="true" />
    </button>
    <div class="dropdown-menu" aria-labelledby="dropdown-toggle-btn-1">
      <h6 class="dropdown-header">Choose script</h6>
        {#each game.scripts as script}
          <button on:click={() => handleLaunch(script)} class="dropdown-item" target="_blank">
            {script}
          </button>
        {/each}
    </div>
  </div>
    -->
  <div slot="buttons">
    {#each game.scripts as script}
      <button on:click={() => handleLaunch(script)} class="btn" target="_blank">
        {script}
      </button>
    {/each}
  </div>
</Card>

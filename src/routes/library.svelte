<script>
  import { localGames, selectedLocalGame, query } from '$lib/store.js';
  import { fly } from 'svelte/transition';

  import Panel from '$lib/library/Panel.svelte';
  import Grid from '$lib/library/Grid.svelte';

  query.subscribe(() => {
    $selectedLocalGame = null;
  });
</script>

<svelte:head>
  <title>Rum - Library</title>
</svelte:head>

<div class="content">
  <div class="row-grid full-height" class:small={$selectedLocalGame !== null}>
    <div class="grid full-height">
      <Grid />
    </div>
  </div>
  {#if $selectedLocalGame !== null}
    <div class="row-panel full-height">
      {#await $localGames}
        Loading...
      {:then games}
        {#key $selectedLocalGame}
          <div
            in:fly={{ y: 100, duration: 300, delay: 300 }}
            out:fly={{ y: 100, duration: 300 }}
            class="full-height panel"
          >
            <Panel
              game={games.filter((game) => game.name.toLowerCase().includes($query.toLowerCase()))[
                $selectedLocalGame
              ]}
              on:close={() => ($selectedLocalGame = null)}
            />
          </div>
        {/key}
      {:catch error}
        <p style="color: red">{error.message}</p>
      {/await}
    </div>
  {/if}
</div>

<style>
  .content {
    height: 100%;
    overflow: hidden;
  }

  .grid {
    overflow-y: auto;
    padding: 10px;
    padding-right: 20px;
    width: 100%;
    max-height: 100%;
  }

  .row-grid {
    padding: 10px;
    height: calc(100vh - 100px);
    max-height: calc(100vh - 100px);
  }

  .row-grid.small {
    max-height: calc(100vh - 320px);
  }

  .row-panel {
    width: 100%;
  }
</style>

<script>
  import { localGames, selectedLocalGame } from '$lib/store.js';
  import { fly } from 'svelte/transition';
  $selectedLocalGame = 0;
  /*
  import Panel from '$lib/library/Panel.svelte';
*/
  import Grid from '$lib/library/Grid.svelte';
</script>

<svelte:head>
  <title>Chad Launcher - Library</title>
</svelte:head>

<div class="content ma-5">
  <div class="col-grid full-height">
    <div
      class="grid full-height pr-3"
      in:fly={{ x: -100, duration: 300, delay: 300 }}
      out:fly={{ x: -100, duration: 300 }}
    >
      <Grid />
    </div>
  </div>
  {#if $selectedLocalGame !== null}
    <div class="col-panel full-height">
      {#await $localGames}
        Loading...
      {:then games}
        {#key $selectedLocalGame}
          <div
            in:fly={{ x: 100, duration: 300, delay: 300 }}
            out:fly={{ x: 100, duration: 300 }}
            class="full-height panel"
          >
            here is le panel
            <!--<Panel game={games[$selectedLocalGame]} on:close={() => ($selectedLocalGame = null)} />-->
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
    display: grid;
    grid-template-columns: auto min-content;
  }

  .full-height {
    height: 100%;
  }

  .grid {
    overflow-y: scroll;
  }

  .col-grid {
    padding: 10px;
  }

  .col-panel {
    width: 400px;
    padding: 10px;
  }

  .panel {
    background-color: var(--secondary);
  }
</style>

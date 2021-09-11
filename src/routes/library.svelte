<script>
  import { localGames, selectedLocalGame } from '$lib/store.js';
  import { fly } from 'svelte/transition';
  $selectedLocalGame = 0;

  import Panel from '$lib/library/Panel.svelte';
  import Grid from '$lib/library/Grid.svelte';
</script>

<svelte:head>
  <title>Chad Launcher - Library</title>
</svelte:head>

<div class="content">
  <div class="row-grid full-height">
    <div
      class="grid full-height"
      in:fly={{ x: -100, duration: 300, delay: 300 }}
      out:fly={{ x: -100, duration: 300 }}
    >
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
            in:fly={{ x: 100, duration: 300, delay: 300 }}
            out:fly={{ x: 100, duration: 300 }}
            class="full-height panel"
          >
            <Panel game={games[$selectedLocalGame]} on:close={() => ($selectedLocalGame = null)} />
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
    grid-template-rows: auto max-content;
  }

  .full-height {
  }

  .grid {
    overflow-y: scroll;
    padding: 10px;
    padding-right: 20px;
    width: 100%;
  }

  .col-grid {
    display: flex;
    padding: 10px;
  }

  .col-panel {
    width: 400px;
  }

  .panel {
    background-color: var(--secondary);
  }
</style>

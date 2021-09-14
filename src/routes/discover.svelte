<script>
  import { genres, selectedGenre, query, page, selectedGame, databaseGames } from '$lib/store.js';
  import { fly } from 'svelte/transition';
  //import Pagination from '$lib/discover/Pagination.svelte';
  import Panel from '$lib/discover/Panel.svelte';
  import Grid from '$lib/discover/Grid.svelte';

  let searchValue = $query;

  let genreItems = [];

  genres.subscribe(async ($genres) => {
    genreItems = ['Any'].concat(await $genres);
  });

  let value = $selectedGenre !== null ? $selectedGenre : 'Any';

  selectedGenre.subscribe((value) => {
    if ($page !== 1 && value !== null) {
      page.set(1);
    }
  });

  $: {
    $selectedGenre = value !== 'Any' ? value : null;
  }
</script>

<svelte:head>
  <title>Chad Launcher - Discover</title>
</svelte:head>

<div class="content">
  <!--
  <div class="row-pagination">
    <div>
      <Pagination />
    </div>
    <div>
      <Select items={genreItems} bind:value />
    </div>

    <div>
      <form
        on:submit={(e) => {
          e.preventDefault();
          $page = 1;
          $query = searchValue;
        }}
      >
        <TextField bind:value={searchValue} placeholder="Search" />
      </form>
    </div>
  </div>
    -->

  <div class="row-grid" class:small={$selectedGame !== null}>
    <div class="grid full-height">
      <Grid />
    </div>
  </div>
  {#if $selectedGame !== null}
    <div class="row-panel">
      {#await $databaseGames}
        Loading...
      {:then games}
        {#key $selectedGame}
          <div
            in:fly={{ y: 100, duration: 300, delay: 300 }}
            out:fly={{ y: 100, duration: 300 }}
            class="full-height"
          >
            <Panel game={games[$selectedGame]} on:close={() => ($selectedGame = null)} />
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
    overflow-y: scroll;
    padding: 10px;
    padding-right: 20px;
    width: 100%;
    max-height: 100%;
  }

  .row-grid {
    padding: 10px;
    height: 100%;
    max-height: calc(100vh - 100px);
  }

  .row-grid.small {
    max-height: calc(100vh - 300px);
  }

  .row-panel {
    width: 100%;
  }
</style>

<script>
  import { mode, genres, selectedGenre, query } from '$lib/store.js';
  import GameGrid from '$lib/GameGrid.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import GameList from '$lib/GameList.svelte';
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';

  let searchValue = '';

  $: invoke('get_genres')
    .then((g) => ($genres = g))
    .catch((err) => console.error(err));

  function handleSelect() {
    const value = this.value;
    if (value !== 'any') {
      $selectedGenre = value;
    } else {
      $selectedGenre = null;
    }
  }
</script>

<svelte:head>
  <title>Chad Launcher - Discover</title>
</svelte:head>

<div class="d-flex flex-wrap">
  <Pagination />

  <div class="content">
    <select class="form-control" id="select-1" on:change={handleSelect}>
      <option value="" selected="selected" disabled="disabled">Filter by genre</option>
      <option value="any">Any</option>
      {#each $genres as genre}
        <option value={genre}>{genre}</option>
      {/each}
    </select>
  </div>

  <div class="content">
    <form
      on:submit={(e) => {
        e.preventDefault();
        $query = searchValue;
      }}
    >
      <input type="text" class="form-control" bind:value={searchValue} placeholder="Search" />
    </form>
  </div>

  {#if $mode == 'list'}
    <GameList />
  {:else if $mode == 'grid'}
    <GameGrid />
  {/if}
</div>

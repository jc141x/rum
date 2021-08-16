<script>
  import { query, genres, selectedGenre, mode } from '$lib/store.js';
  import { onMount } from 'svelte';
  onMount(async () => {
    const { invoke } = await import('@tauri-apps/api/tauri');
    invoke('get_genres')
      .then((g) => ($genres = g))
      .catch((err) => console.error(err));
  });
</script>

<div class="sidebar section">
  <div class="sidebar-menu">
    <!-- Sidebar content with the search box -->
    <div class="sidebar-content">
      <input type="text" class="form-control" bind:value={$query} placeholder="Search" />
      <div class="mt-10 font-size-12">
        <!-- mt-10 = margin-top: 1rem (10px), font-size-12 = font-size: 1.2rem (12px) -->
        <!--  Press <kbd>/</kbd> to focus -->
      </div>
    </div>
    <!-- Sidebar links and titles -->
    <h5 class="sidebar-title">View Mode</h5>
    <div class="sidebar-divider" />
    <a href="" on:click={() => ($mode = 'list')} class="sidebar-link" class:active={$mode == 'list'}
      >List</a
    >
    <a href="" on:click={() => ($mode = 'grid')} class="sidebar-link" class:active={$mode == 'grid'}
      >Grid</a
    >
    <br />
    <h5 class="sidebar-title">Genres</h5>
    <div class="sidebar-divider" />
    {#each [...$genres].sort() as genre (genre.id)}
      <a
        href=""
        on:click={() => ($selectedGenre = genre)}
        class:active={$selectedGenre == genre}
        class="sidebar-link">{genre.name}</a
      >
    {/each}
  </div>
</div>

<style>
</style>

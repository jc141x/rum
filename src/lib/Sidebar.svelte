<script>
  import { query, genres, selectedGenre, mode } from '$lib/store.js';
  const url = 'https://bkftwbhopivmrgzcagus.supabase.co/rest/v1/genre?select=id,name';
  import axios from 'axios';
  axios
    .get(url, {
      headers: {
        apikey:
          'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyNzY0NDc0OCwiZXhwIjoxOTQzMjIwNzQ4fQ.MheXAiuWYFGDuFhfzAnANMzJU2UU4HN2dxwMxGdQd5A'
      }
    })
    .then((response) => ($genres = response.data))
    .catch((error) => console.log(error));
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
    {#each [...$genres].sort((x1, x2) => {
      if (x1.name > x2.name) return 1;
      if (x1.name < x2.name) return -1;
      return 0;
    }) as genre (genre.id)}
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

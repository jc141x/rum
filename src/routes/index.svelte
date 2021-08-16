<script context="module">
  export const prerender = true;
</script>

<script>
  import { mode, games } from '$lib/store.js';
  import Sidebar from '$lib/Sidebar.svelte';
  import GameGrid from '$lib/GameGrid.svelte';
  import Pagination from '$lib/Pagination.svelte';
  const url = 'https://bkftwbhopivmrgzcagus.supabase.co/rest/v1/game?select=*';
  import axios from 'axios';
  import GameList from '$lib/GameList.svelte';
  import GameCard from '$lib/GameCard.svelte';
  axios
    .get(url, {
      headers: {
        apikey:
          'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyNzY0NDc0OCwiZXhwIjoxOTQzMjIwNzQ4fQ.MheXAiuWYFGDuFhfzAnANMzJU2UU4HN2dxwMxGdQd5A'
      }
    })
    .then((response) => ($games = response.data))
    .catch((error) => console.log(error));
</script>

<svelte:head>
  <title>Home | GNU/Linux P2P Pirates</title>
</svelte:head>

<div class="container-fluid">
  <Pagination />
  {#if $mode == 'list'}
    <GameList />
  {:else if $mode == 'grid'}
    <GameGrid />
  {/if}
</div>

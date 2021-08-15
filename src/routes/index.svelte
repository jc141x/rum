<script context="module">
  export const prerender = true;
</script>

<script>
  import { page, games, selectedGame } from '$lib/store.js';
  import Sidebar from '$lib/Sidebar.svelte';
  import GameCard from '$lib/GameCard.svelte';
  import Pagination from '$lib/Pagination.svelte';
  const url = 'https://bkftwbhopivmrgzcagus.supabase.co/rest/v1/game?select=*';
  import axios from 'axios';
  axios
    .get(url, {
      headers: {
        apikey:
          'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyNzY0NDc0OCwiZXhwIjoxOTQzMjIwNzQ4fQ.MheXAiuWYFGDuFhfzAnANMzJU2UU4HN2dxwMxGdQd5A'
      }
    })
    .then((response) => ($games = response.data))
    .catch((error) => console.log(error));

  $: getGridGames = () => {
    let _games = [];
    const n = 3;
    for (let i = ($page - 1) * 20; i < $page * 20; i += n) {
      _games.push($games.slice(i, i + n));
    }
    return _games;
  };
</script>

<svelte:head>
  <title>Home</title>
</svelte:head>

<div class="columns p-3">
  <Sidebar />
  <main class="column">
    <div class="tile is-ancestor">
      <div class="tile is-parent">
        <div class="tile is-child">
          <Pagination />
        </div>
      </div>
    </div>
    {#each getGridGames() as games}
      <div class="tile is-ancestor">
        {#each games as game (game.id)}<GameCard {game} />{/each}
      </div>
    {/each}
  </main>
</div>

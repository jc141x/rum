<script>
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import { games, page, query, selectedGenre } from '$lib/store.js';
  import GameCard from './GameCard.svelte';

  $: {
    let opts = { page_number: $page - 1, page_size: 20 };

    if ($query != '') {
      opts.search = $query;
    }
    if ($selectedGenre != '') {
      opts.filter_genre = $selectedGenre;
    }

    invoke('get_games', { opts })
      .then((g) => ($games = g))
      .catch((err) => console.error(err));
  }
</script>

<div class="grid">
  {#each $games as game (game.id)}
    <GameCard {game} />
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(300px, 100%), 1fr));
    gap: 1rem;
  }
</style>

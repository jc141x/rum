<script>
  import command from '$lib/command';
  import { games, page, query, selectedGenre } from '$lib/store.js';
  import GameCard from './GameCard.svelte';
  import DownloadGameModal from '$lib/DownloadGameModal.svelte';

  let downloadGame = null;

  $: {
    let opts = { page_number: $page - 1, page_size: 20 };

    if ($query != '') {
      opts.search = $query;
    }
    if ($selectedGenre != '') {
      opts.filter_genre = $selectedGenre;
    }

    command
      .database('get_games', { opts })
      .then((g) => ($games = g))
      .catch((err) => console.error(err));
  }
</script>

<div>
  <div class="grid">
    {#each $games as game (game.id)}
      <GameCard {game} on:download={() => (downloadGame = game)} />
    {/each}
  </div>

  {#if downloadGame !== null}
    <DownloadGameModal game={downloadGame} on:close={() => (downloadGame = null)} />
  {/if}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(300px, 100%), 1fr));
    gap: 1rem;
  }
</style>

<script>
  import command from '$lib/command';
  import { page, query, selectedGenre, databaseGames, selectedGame } from '$lib/store.js';
  import GameCard from './GameCard.svelte';
  import DownloadGameModal from '$lib/DownloadGameModal.svelte';
  import { ProgressCircular } from 'svelte-materialify/src';

  let downloadGame = null;
</script>

<div>
  <div class="grid">
    {#await $databaseGames}
      <ProgressCircular indeterminate color="primary" />
    {:then games}
      {#each games as game, i (game.id)}
        <GameCard
          {game}
          on:download={() => (downloadGame = game)}
          on:click={() => selectedGame.set(i)}
        />
      {/each}
    {:catch error}
      <p style="color: red">{error.message}</p>
    {/await}
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

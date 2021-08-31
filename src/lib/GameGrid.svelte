<script>
  import command from '$lib/command';
  import { page, query, selectedGenre, databaseGames, selectedGame } from '$lib/store.js';
  import GameCard from './GameCard.svelte';
  import DownloadGameModal from '$lib/DownloadGameModal.svelte';
  import { ProgressCircular } from 'svelte-materialify/src';

  let downloadGame = null;
</script>

<div class="full">
  {#await $databaseGames}
    <div class="d-flex justify-center align-center full">
      <ProgressCircular indeterminate color="primary" />
    </div>
  {:then games}
    <div class="grid full">
      {#each games as game, i (game.id)}
        <GameCard
          {game}
          selected={$selectedGame == i}
          on:download={() => (downloadGame = game)}
          on:click={() => selectedGame.set(i)}
        />
      {/each}
    </div>
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}

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

  .full {
    width: 100%;
    height: 100%;
  }
</style>

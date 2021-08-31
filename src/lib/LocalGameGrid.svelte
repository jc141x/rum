<script>
  import { localGames, selectedLocalGame } from '$lib/store.js';
  import LocalGameCard from './LocalGameCard.svelte';
  import { ProgressCircular } from 'svelte-materialify/src';
</script>

{#await $localGames}
  <div class="d-flex justify-center align-center full">
    <ProgressCircular indeterminate color="primary" />
  </div>
{:then games}
  <div class="grid full">
    {#each games as game, i (game.id)}
      <LocalGameCard
        {game}
        selected={$selectedLocalGame == i}
        on:click={() => selectedLocalGame.set(i)}
      />
    {/each}
  </div>
{:catch error}
  <p style="color: red">{error.message}</p>
{/await}

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

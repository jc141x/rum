<script>
  import { localGames, selectedLocalGame } from '$lib/store.js';
  import LocalGameCard from './LocalGameCard.svelte';
  import { ProgressCircular } from 'svelte-materialify/src';
</script>

<div class="grid">
  {#await $localGames}
    <ProgressCircular indeterminate color="primary" />
  {:then games}
    {#each games as game, i (game.id)}
      <LocalGameCard
        {game}
        selected={$selectedLocalGame == i}
        on:click={() => selectedLocalGame.set(i)}
      />
    {/each}
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(300px, 100%), 1fr));
    gap: 1rem;
  }
</style>

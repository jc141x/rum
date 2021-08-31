<script>
  import { databaseGames, selectedGame } from '$lib/store.js';
  import Card from '$lib/discover/Card.svelte';
  import Grid from '$lib/Grid.svelte';
  import { ProgressCircular } from 'svelte-materialify/src';
</script>

{#await $databaseGames}
  <div class="d-flex justify-center align-center full">
    <ProgressCircular indeterminate color="primary" />
  </div>
{:then games}
  <Grid>
    {#each games as game, i (game.id)}
      <Card {game} selected={$selectedGame == i} on:click={() => selectedGame.set(i)} />
    {/each}
  </Grid>
{:catch error}
  <p style="color: red">{error.message}</p>
{/await}
<div class="full">
  {#await $databaseGames}
    <div class="d-flex justify-center align-center full">
      <ProgressCircular indeterminate color="primary" />
    </div>
  {:then games}
    <Grid>
      {#each games as game, i (game.id)}
        <Card
          {game}
          selected={$selectedGame == i}
          on:click={() => selectedGame.set(i)}
        />
      {/each}
    </Grid>
  {:catch error}
    <p style="color: red">{error.message}</p>
  {/await}
</div>

<style>
  .full {
    width: 100%;
    height: 100%;
  }
</style>

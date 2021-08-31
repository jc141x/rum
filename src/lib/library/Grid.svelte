<script>
  import { localGames, selectedLocalGame } from '$lib/store.js';
  import Card from '$lib/library/Card.svelte';
  import Grid from '$lib/Grid.svelte';
  import { ProgressCircular } from 'svelte-materialify/src';
</script>

{#await $localGames}
  <div class="d-flex justify-center align-center full">
    <ProgressCircular indeterminate color="primary" />
  </div>
{:then games}
  <Grid>
    {#each games as game, i (game.id)}
      <Card {game} selected={$selectedLocalGame == i} on:click={() => selectedLocalGame.set(i)} />
    {/each}
  </Grid>
{:catch error}
  <p style="color: red">{error.message}</p>
{/await}

<style>
  .full {
    width: 100%;
    height: 100%;
  }
</style>

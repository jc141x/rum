<script>
  import { databaseGames, selectedGame } from '$lib/store.js';
  import Card from '$lib/discover/Card.svelte';
  import Grid from '$lib/Grid.svelte';
  import { styles } from '$lib/styles';
  import { Pulse } from 'svelte-loading-spinners';
</script>

{#await $databaseGames}
  <div class="center">
    <Pulse size="60" color={$styles.primary} unit="px" duration="1s" />
  </div>
{:then games}
  <Grid>
    {#each games as game, i (game.hash)}
      <Card
        {game}
        selected={$selectedGame == i}
        on:click={() => {
          selectedGame.set(i);
        }}
      />
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

  .center {
    display: flex;
    justify-content: center;
    align-content: center;
    margin-top: 200px;
  }
</style>

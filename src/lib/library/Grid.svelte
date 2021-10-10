<script>
  import { localGames, selectedLocalGame, query } from '$lib/store.js';
  import { styles } from '$lib/styles';
  import { Pulse } from 'svelte-loading-spinners';
  import Card from '$lib/library/Card.svelte';
  import Grid from '$lib/Grid.svelte';
</script>

{#await $localGames}
  <div class="center">
    <Pulse size="60" color={$styles.primary} unit="px" duration="1s" />
  </div>
{:then games}
{#key $query}
  <Grid>
    {#each games.filter(game => game.name.toLowerCase().includes($query.toLowerCase())) as game, i (game.id)}
      <Card
        {game}
        selected={$selectedLocalGame == i}
        on:click={(e) => {
          selectedLocalGame.set(i);
        }}
      />
    {/each}
  </Grid>
{/key}
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

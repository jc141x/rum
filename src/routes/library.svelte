<script>
  import { localGames, selectedLocalGame, query } from '$lib/store.js';
  import { fly } from 'svelte/transition';

  import Panel from '$lib/library/Panel.svelte';
  import Grid from '$lib/library/Grid.svelte';

  query.subscribe(() => {
    $selectedLocalGame = null;
  });
</script>

<svelte:head>
  <title>Rum - Library</title>
</svelte:head>

<Grid />

{#if $selectedLocalGame !== null}
    {#await $localGames}
      Loading...
    {:then games}
      {#key $selectedLocalGame}
          <Panel
            game={games.filter((game) => game.name.toLowerCase()?.includes($query.toLowerCase()))[
              $selectedLocalGame
            ]}
            on:close={() => ($selectedLocalGame = null)}
          />
      {/key}
    {:catch error}
      <p style="color: red">{error.message}</p>
    {/await}
{/if}

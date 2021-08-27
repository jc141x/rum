<script>
  import command from '$lib/command';
  import { page, localGames, config } from '$lib/store.js';
  import LocalGameCard from './LocalGameCard.svelte';

  $: {
    if ($config) {
      console.log('update library');
      command.library('reload_games');
      command
        .library('get_games')
        .then((g) => {
          $localGames = g;
          console.log(g);
        })
        .catch((err) => console.error(err));
    }
  }
</script>

<div class="grid">
  {#each $localGames as game (game.id)}
    <LocalGameCard {game} />
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(300px, 100%), 1fr));
    gap: 1rem;
  }
</style>

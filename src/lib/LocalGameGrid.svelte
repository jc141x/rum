<script>
  import command from '$lib/command';
  import { localGames, config } from '$lib/store.js';
  import LocalGameCard from './LocalGameCard.svelte';

  config.subscribe(async () => {
    await command.library('reload_games');
    $localGames = await command.library('get_games');
  });
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

<script>
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import { page, localGames } from '$lib/store.js';
  import LocalGameCard from './LocalGameCard.svelte';

  invoke('get_local_games')
    .then((g) => ($localGames = g))
    .catch((err) => console.error(err));
</script>

<div class="flex-container">
  {#each $localGames as game (game.id)}
    <LocalGameCard {game} />
  {/each}
</div>

<style>
  .flex-container {
    display: flex;
    flex-wrap: wrap;
  }
</style>

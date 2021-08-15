<script>
  import { page, filteredGames, selectedGame } from '$lib/store.js';
  import GameCard from './GameCard.svelte';
  $: getGridGames = () => {
    let _games = [];
    const n = 3;
    for (let i = ($page - 1) * 20; i < $page * 20; i += n) {
      _games.push($filteredGames.slice(i, i + n));
    }
    return _games;
  };
</script>

{#each getGridGames() as games}
  <div class="tile is-ancestor">
    {#each games as game (game.id)}<GameCard {game} />{/each}
  </div>
{/each}

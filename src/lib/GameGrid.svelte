<script>
  import { page, games } from '$lib/store.js';
  import GameCard from './GameCard.svelte';
</script>

<div class="row row-eq-spacing">
  {#each $games.slice(($page - 1) * 20, $page * 20) as game (game.id)}
    <div class="col-lg-3 col-md-4 col-sm-12">
      <GameCard {game} />
    </div>
  {/each}
</div>

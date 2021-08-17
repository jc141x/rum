<style>
.flex-container {
    display: flex;
    flex-wrap: wrap;
}
</style>

<script>
  import { page, games } from '$lib/store.js';
  import GameCard from './GameCard.svelte';
</script>

<!--
<div class="row row-eq-spacing">
  {#each $games as game (game.id)}
    <div class="col-lg-3 col-md-4 col-sm-12">
      <GameCard {game} />
    </div>
  {/each}
</div>
    -->

<div class="flex-container">
  {#each $games as game (game.id)}
    <GameCard {game} />
  {/each}
</div>


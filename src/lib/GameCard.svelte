<script>
  import { selectedGame } from '$lib/store.js';
  import banner from './default.png';
  export let game;
  function get_banner(game) {
    return game.banner_rel_path === null
      ? banner
      : `https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/${game.banner_rel_path}`;
  }
</script>

<div class="tile is-parent">
  <div class="tile is-child">
    <div class="card" class:has-background-primary={$selectedGame == game}>
      <div class="card-image">
        <figure class="image">
          <img src={get_banner(game)} alt={game.name} />
        </figure>
      </div>
      <div class="card-content">
        <p class="title is-4">{game.name}</p>
        <p class="subtitle is-7 has-text-grey">{game.hash}</p>

        <div class="content">
          {game.description}
        </div>
      </div>
    </div>
  </div>
</div>

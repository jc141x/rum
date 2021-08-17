<style>
    .description {
        text-overflow: ellipsis;
        display: -webkit-box;
        -webkit-line-clamp: 3;
        -webkit-box-orient: vertical;
    }
</style>

<script>
  import { selectedGame } from '$lib/store.js';
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import banner from './default.png';

  export let game;
  function get_banner(game) {
    return game.banner_path === null
      ? banner
      : `https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/${game.banner_path}`;
  }
  function toTitleCase(str) {
    return str.replace(/\w\S*/g, function (txt) {
      return txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase();
    });
  }

  const handleMagnet = () => {
    invoke('open_magnet', { game });
  };
</script>

<div class="card p-0 flex-fill w-250 m-10 d-flex flex-column">
  <!-- Image -->
  <img class="img-fluid rounded-top w-full mh-full" src={get_banner(game)} alt={game.name} />
  <div class="content m-15 h-250 overflow-hidden mh-full d-flex flex-column">
    <!-- Title -->
    <header class="font-size-18 font-weight-bold">
      {game.name}
    </header>
    <p class="text-muted my-0">
      {toTitleCase(game.type)}
    </p>
    <!-- Genres -->
    <div class="">
    {#each game.genres as genre}
      <span class="badge"> {genre} </span>
    {/each}
    </div>
    {#if game.nsfw}
      <span class="badge badge-danger"> 18+ </span>
    {/if}
    <!-- Description -->
    <p class="description flex-shrink-1 overflow-hidden">
      {game.description}
    </p>
    <!-- Download -->
    <div class="text-right mt-auto">
      <button on:click={() => handleMagnet()} class="btn" target="_blank">Download</button>
    </div>
  </div>
</div>

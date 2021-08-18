<script context="module">
  export const prerender = true;
  export const ssr = false;
</script>

<script>
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import { listen } from '../../node_modules/@tauri-apps/api/event';
  import { mode } from '$lib/store.js';
  import Sidebar from '$lib/Sidebar.svelte';
  import GameGrid from '$lib/GameGrid.svelte';
  import LocalGameGrid from '$lib/LocalGameGrid.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import GameList from '$lib/GameList.svelte';

  /*
  invoke('get_local_games')
    .then(console.log)
    .catch((err) => console.error(err));

  invoke('run_game', { index: 7 })
    .then(console.log)
    .catch((err) => console.error(err));

  invoke('open_terminal', { index: 7 })
    .then(console.log)
    .catch((err) => console.error(err));

  listen('game_log', (event) => {
    // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
    // event.payload is the payload object
    console.log(event.payload);
  });
*/
</script>

<svelte:head>
  <title>Home | GNU/Linux P2P Pirates</title>
</svelte:head>

<div class="d-flex flex-wrap">
  <Pagination />
  {#if $mode == 'list'}
    <GameList />
  {:else if $mode == 'grid'}
    <LocalGameGrid />
  {/if}
</div>

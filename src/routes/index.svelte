<script context="module">
    export const prerender = true;
    export const ssr = false;
</script>

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { mode, games, page } from '$lib/store.js';
    import Sidebar from '$lib/Sidebar.svelte';
    import GameGrid from '$lib/GameGrid.svelte';
    import Pagination from '$lib/Pagination.svelte';
    import GameList from '$lib/GameList.svelte';

    $: invoke('get_games', { opts: { page_number: $page, page_size: 20 }})
        .then(g => $games = g)
        .catch(err => console.error(err));
</script>

<svelte:head>
    <title>Home | GNU/Linux P2P Pirates</title>
</svelte:head>

<div class="container-fluid">
  <Pagination />
  {#if $mode == 'list'}
    <GameList />
  {:else if $mode == 'grid'}
    <GameGrid />
  {/if}
</div>

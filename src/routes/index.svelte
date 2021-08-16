<script context="module">
    export const prerender = true;
    export const ssr = false;
</script>

<script>
    import { invoke } from "@tauri-apps/api/tauri";
    import { mode, games, page, query, selectedGenre } from '$lib/store.js';
    import Sidebar from '$lib/Sidebar.svelte';
    import GameGrid from '$lib/GameGrid.svelte';
    import Pagination from '$lib/Pagination.svelte';
    import GameList from '$lib/GameList.svelte';

    $: {
        let opts = { page_number: $page - 1, page_size: 20 };

        if ($query != "") {
            opts.search = $query;
        } 
        if ($selectedGenre != "") {
            opts.filter_genre = $selectedGenre;
        } 

        invoke('get_games', { opts })
            .then(g => $games = g)
            .catch(err => console.error(err));
    }
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

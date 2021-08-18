<script>
  import { mode } from '$lib/store.js';
  import Sidebar from '$lib/Sidebar.svelte';
  import LocalGameGrid from '$lib/LocalGameGrid.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import GameList from '$lib/GameList.svelte';
</script>

<svelte:head>
  <title>Chad Launcher - Library</title>
</svelte:head>

<div class="d-flex flex-wrap">
  {#if $mode == 'list'}
    <GameList /> <!-- TODO Make local version -->
  {:else if $mode == 'grid'}
    <LocalGameGrid />
  {/if}
</div>

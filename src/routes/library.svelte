<script>
  import { mode } from '$lib/store.js';
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import LocalGameGrid from '$lib/LocalGameGrid.svelte';

</script>

<svelte:head>
  <title>Chad Launcher - Library</title>
</svelte:head>

<div class="d-flex flex-wrap">
  <LocalGameGrid />
</div>

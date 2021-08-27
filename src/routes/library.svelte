<script>
  import { mode } from '$lib/store.js';
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import LocalGameGrid from '$lib/LocalGameGrid.svelte';
  import { Container } from 'svelte-materialify';

  invoke('reload_local_games');
</script>

<svelte:head>
  <title>Chad Launcher - Library</title>
</svelte:head>

<Container fluid>
  <LocalGameGrid />
</Container>

<script context="module">
  export const prerender = true;
  export const ssr = false;
</script>

<script>
  import { isAdmin } from '$lib/store';
</script>

<svelte:head>
  <title>Chad Launcher</title>
</svelte:head>

<div>
  Welcome to Chad Launcher! <br /><br />

  Start by going opening the menu and going to the settings page to configure library paths and
  torrent clients. <br /><br />
</div>

<style>
  div {
    margin: 10px;
  }
</style>

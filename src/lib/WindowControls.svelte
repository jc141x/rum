<script>
  import { mdiWindowClose, mdiWindowMaximize, mdiWindowMinimize } from '@mdi/js';

  import Icon from 'mdi-svelte';
  import { onMount } from 'svelte';

  let appWindow = null;
  export let inverted = false;

  onMount(async () => {
    appWindow = (await import('../../node_modules/@tauri-apps/api/window')).appWindow;
  });
</script>

<div class="top">
  {#if inverted}
    <button on:click={appWindow.close()}><Icon path={mdiWindowClose} /></button>
  {:else}
    <button on:click={appWindow.minimize()}><Icon path={mdiWindowMinimize} /></button>
  {/if}
  <button on:click={appWindow.toggleMaximize()}><Icon path={mdiWindowMaximize} /></button>
  {#if inverted}
    <button on:click={appWindow.minimize()}><Icon path={mdiWindowMinimize} /></button>
  {:else}
    <button on:click={appWindow.close()}><Icon path={mdiWindowClose} /></button>
  {/if}
</div>

<style>
  .top {
    min-width: 150px;
  }
</style>

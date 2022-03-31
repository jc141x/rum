<script>
  import IconWindowClose from '~icons/mdi/window-close';
  import IconWindowMaximize from '~icons/mdi/window-maximize';
  import IconWindowMinimize from '~icons/mdi/window-minimize';

  import { onMount } from 'svelte';

  let appWindow = null;
  export let inverted = false;

  onMount(async () => {
    appWindow = (await import('../../node_modules/@tauri-apps/api/window')).appWindow;
  });
</script>

  {#if inverted}
    <button class="button clear icon-only" on:click={appWindow.close()}><IconWindowClose /></button>
  {:else}
    <button class="button clear icon-only" on:click={appWindow.minimize()}><IconWindowMinimize /></button>
  {/if}
  <button class="button clear icon-only" on:click={appWindow.toggleMaximize()}><IconWindowMaximize /></button>
  {#if inverted}
    <button class="button clear icon-only" on:click={appWindow.minimize()}><IconWindowMinimize /></button>
  {:else}
    <button class="button clear icon-only" on:click={appWindow.close()}><IconWindowClose /></button>
  {/if}

<style>
  button.button {
    margin: 0
  }
</style>
<script>
  import { decorations } from '$lib/store.js';
  import { onMount } from 'svelte';
  import  Logo from '$lib/assets/Logo.svelte';
  import Icon from 'mdi-svelte';
  import {mdiBookshelf, mdiCog, mdiBookOpenVariant } from '@mdi/js';
  import WindowControls from './WindowControls.svelte';

  onMount(() => {
    decorations.subscribe(async (value) => {
      let { appWindow } = await import('../../node_modules/@tauri-apps/api/window');

      if (value != 'system') {
        await appWindow.setDecorations(false);
      } else {
        await appWindow.setDecorations(true);
      }
    });
  });
</script>

<div class="header">
  {#if $decorations == 'left'}
    <WindowControls inverted />
  {/if}
  <a href="/" class="link"><Logo width="32pt" /></a>
  <div class="links">
    <a href="/library" class="link"><Icon path={mdiBookshelf}/></a>
    <a href="/settings" class="link"><Icon path={mdiCog}/></a>
    <a href="/wiki" class="link"><Icon path={mdiBookOpenVariant}/></a>
  </div>
  {#if $decorations == 'right'}
    <div class="right">
      <WindowControls />
    </div>
  {/if}
</div>

<style>
  .header {
    position: sticky;
    top: 0;
    width: 100%;
    z-index: 100;
    display: flex;
    flex-direction: row;
    justify-content: left;
    align-items: center;
    color: var(--primary);
    overflow-x: hidden;
  }

  .links {
    display: flex;
    flex-direction: row;
    align-items: center;
    overflow-x: auto;
  }
  .links::-webkit-scrollbar {
    height: 4px;
  }

  .link {
    text-decoration: none;
  }

  .header > *,
  .links > * {
    margin: 10px;
  }

  .right {
    margin-left: auto;
  }

</style>

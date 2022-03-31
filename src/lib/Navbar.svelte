<script>
  import { decorations, query } from '$lib/store.js';
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import WindowControls from './WindowControls.svelte';
  import Search from '$lib/library/Search.svelte';

  import IconBookshelf from '~icons/mdi/bookshelf';
  import IconCog from '~icons/mdi/cog';
  import IconBookOpenVariant from '~icons/mdi/book-open-variant';
  
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

<header data-tauri-drag-region>
  <nav class="nav" data-tauri-drag-region>
    {#if $decorations == 'left'}
      <div class="nav-left" data-tauri-drag-region>
        <WindowControls inverted />
      </div>
    {/if}
    <div class={$decorations == 'left' ? 'nav-center' : 'nav-left'} data-tauri-drag-region>      
      <a href="/library" class="button icon-only clear"><IconBookshelf/></a>
      <a href="/settings" class="button icon-only clear"><IconCog/></a>
      <a href="/wiki" class="button icon-only clear"><IconBookOpenVariant/></a>
      {#if $page.url.pathname == '/library'}
        <div class="button icon-only clear search"><Search bind:query={$query} /></div>
      {/if}
    </div>
    <div class="nav-right" data-tauri-drag-region>
        {#if $decorations == 'right'}
        <WindowControls />
        {/if}
      </div>
  </nav>
  </header>

  <style>
    .nav {
      width: 100%;
    }
    .nav a {
      margin: 0;
    }
  </style>

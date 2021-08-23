<script>
  import Header from '$lib/Header.svelte';
  import Index from './index.svelte';
  import Library from './library.svelte';
  import Discover from './discover.svelte';
  import Documentation from './documentation.svelte';
  import { MaterialApp, Button, Window, WindowItem } from 'svelte-materialify/src';
  let theme = 'dark';
  let value = 0;
</script>

<MaterialApp {theme}>
  <Header bind:value />
  <Window {value} class="pa-4">
    <WindowItem>
      <Index />
    </WindowItem>
    <WindowItem>
      <Library />
    </WindowItem>
    <WindowItem>
      <Discover />
    </WindowItem>
    <WindowItem>
      <Documentation />
    </WindowItem>
  </Window>
</MaterialApp>

<style>
  :global(.content-wrapper)::-webkit-scrollbar {
    width: 1rem;
  }

  :global(.content-wrapper)::-webkit-scrollbar-track {
    background-color: #25282c;
    border-radius: 0px;
  }

  :global(.content-wrapper)::-webkit-scrollbar-thumb {
    background-color: #191c20;
    border-radius: 0px;
  }
  :global(.sidebar)::-webkit-scrollbar {
    width: 1rem;
  }

  :global(.sidebar)::-webkit-scrollbar-track {
    background-color: #25282c;
    border-radius: 0px;
  }

  :global(.sidebar)::-webkit-scrollbar-thumb {
    background-color: #191c20;
    border-radius: 0px;
  }
</style>

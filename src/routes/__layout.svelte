<script>
    import "../global.scss";
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

<style lang="scss">
  @import 'svelte-materialify/src/styles/variables';

  :global(body) {
    background-color: map-get($material-dark-theme, "surface");
  }
</style>

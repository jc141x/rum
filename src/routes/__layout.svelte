<script>
  import 'roboto-fontface-woff/css/roboto/sass/roboto-fontface.scss';
  import '../global.scss';

  import Navbar from '$lib/Navbar.svelte';
  import Sidebar from '$lib/Sidebar.svelte';
  import Header from '$lib/Header.svelte';
  import Index from './index.svelte';
  import Library from './library.svelte';
  import Discover from './discover.svelte';
  import Documentation from './documentation.svelte';
  import { MaterialApp, Button, Window, WindowItem } from 'svelte-materialify/src';
  import Settings from './settings.svelte';
  import command from '$lib/command';
  import { load } from '$lib/store';

  let theme = 'dark';
  let value = 0;

  load();
</script>

<MaterialApp {theme}>
  <Navbar />
  <Sidebar />
  <slot />
</MaterialApp>

<style lang="scss">
  @import 'svelte-materialify/src/styles/variables';

  :global(body) {
    background-color: map-get($material-dark-theme, 'surface');
  }
</style>

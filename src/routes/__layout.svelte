<script context="module">
  export const load = async ({ page }) => ({
    props: {
      key: page.path
    }
  });
</script>

<script>
  import '../app.postcss';

  import GlobalKeyBinds from '$lib/GlobalKeyBinds.svelte';
  import Navbar from '$lib/Navbar.svelte';
  import { load as loadStore } from '$lib/store';
  import ThemeProvider from '$lib/ThemeProvider.svelte';
  import PageTransition from '$lib/PageTransition.svelte';
  import command from '$lib/command';
  import { getCurrent } from '../../node_modules/@tauri-apps/api/window'

  export let key;

  loadStore();
  const win = getCurrent();
  (async () => command.misc('init_bg_process', { win }))();

</script>
<GlobalKeyBinds />
<ThemeProvider>
  <div class="wrapper">
    <Navbar />
    <PageTransition refresh={key}>
      <slot />
    </PageTransition>
  </div>
</ThemeProvider>

<style>
  .wrapper {
    display: grid;
    grid-template-rows: minmax(0, min-content) auto;
    height: 100vh;
    padding-inline: 10px;
    overflow: auto;
  }
</style>

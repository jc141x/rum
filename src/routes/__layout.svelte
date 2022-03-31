<script context="module">
  export const load = async ({ url, params }) => ({
    props: {
      key: url
    }
  });
</script>

<script>
  //import '../app.postcss';
  import '../app.scss';

  // disable for now import GlobalKeyBinds from '$lib/GlobalKeyBinds.svelte';
  import Navbar from '$lib/Navbar.svelte';
  import { load as loadStore } from '$lib/store';
  import ThemeProvider from '$lib/ThemeProvider.svelte';
  import PageTransition from '$lib/PageTransition.svelte';
  import command from '$lib/command';
  import { getCurrent } from '../../node_modules/@tauri-apps/api/window';

  export let key;

  loadStore();
  const win = getCurrent();
  (async () => command.misc('init_bg_process', { win }))();
</script>

<!--<GlobalKeyBinds /> -->
<Navbar />
<PageTransition refresh={key}>
  <slot />
</PageTransition>

<script context="module">
  export const load = async ({ page }) => ({
    props: {
      key: page.path
    }
  });
</script>

<script>
  import '../app.postcss';

  import Navbar from '$lib/Navbar.svelte';
  import { load as loadStore } from '$lib/store';
  import ThemeProvider from '$lib/ThemeProvider.svelte';
  import PageTransition from '$lib/PageTransition.svelte';

  export let key;

  loadStore();
</script>

<ThemeProvider>
  <div class="wrapper">
    <Navbar />
    <!--<Sidebar />-->
    <PageTransition refresh={key}>
      <slot />
    </PageTransition>
  </div>
</ThemeProvider>

<style>
  .wrapper {
    display: grid;
    grid-template-rows: min-content auto;
    height: 100vh;
    padding: 10px;
    overflow: hidden;
  }
</style>

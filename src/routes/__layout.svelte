<script>
  import '../app.postcss';
  //import 'roboto-fontface-woff/css/roboto/sass/roboto-fontface.scss';
  import '../global.scss';

  import Navbar from '$lib/Navbar.svelte';
  import { load } from '$lib/store';
  import ThemeProvider from '$lib/ThemeProvider.svelte';

  load();
</script>

<ThemeProvider>
  <div class="wrapper">
    <Navbar />
    <!--<Sidebar />-->
    <slot />
  </div>
</ThemeProvider>

<style>
  .wrapper {
    display: grid;
    grid-template-rows: min-content auto;
    height: 100%;
    padding: 10px;
  }
</style>

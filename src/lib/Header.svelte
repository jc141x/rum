<script>
  import { query } from './store.js';
  let navVisible = false;
</script>

<nav class="navbar">
    <!--
  <div class="navbar-content">
    <button class="btn btn-action" type="button">
      <span class="material-icons d-block">menu</span>
      <span class="sr-only">Toggle sidebar</span>
    </button>
  </div>
    -->
  <a href="/" class="navbar-brand">
    <span>Chad Launcher</span>
  </a>
  <ul class="navbar-nav d-none d-md-flex">
    <li class="nav-item" class:active={false}>
      <a href="/library" class="nav-link">Library</a>
    </li>
    <li class="nav-item" class:active={false}>
      <a href="/discover" class="nav-link">Discover</a>
    </li>
    <li class="nav-item" class:active={false}>
      <a href="/documentation" class="nav-link">Documentation</a>
    </li>
  </ul>
    <!--
  <form class="form-inline d-none d-md-flex ml-auto" action="..." method="...">
    <input type="text" bind:value={$query} class="form-control" placeholder="Search" />
    <button class="btn btn-primary">Search</button>
  </form>
    -->
</nav>

<style>
</style>

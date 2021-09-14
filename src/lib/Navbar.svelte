<script>
  import { mdiMenu, mdiMagnify } from '@mdi/js';
  import { query } from './store.js';
</script>

<div class="header">
  <h3 class="title"><a href="/" class="link">Chad Launcher</a></h3>
  <input bind:value={$query} style="max-width:300px" dense rounded filled placeholder="Search" />
  <div class="links">
    <a href="/library" class="link">Library</a>
    <a href="/discover" class="link">Discover</a>
    <a href="/downloads" class="link">Downloads</a>
    <a href="/settings" class="link">Settings</a>
    <a href="/documentation" class="link">Dependencies</a>
    <a href="/about" class="link">About</a>
  </div>
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

  .title {
    min-width: 200px;
  }

  .links {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
  }

  input {
    background-color: var(--secondary);
    border: 2px solid var(--primary);
    flex-shrink: 1;
  }

  .link {
    text-decoration: none;
  }

  .header > *,
  .links > * {
    margin: 10px;
  }

  .grow {
    flex-grow: 1;
  }
</style>

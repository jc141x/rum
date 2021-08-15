<script>
  import { query } from './store.js';
  let navVisible = false;
</script>

<header>
  <nav class="is-fixed-top is-spaced navbar" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
      <a class="navbar-item has-text-weight-bold is-size-4" href="/"> GNU/Linux P2P Pirates </a>
      <a
        on:click={() => {
          navVisible = !navVisible;
        }}
        role="button"
        class="navbar-burger"
        aria-label="menu"
        aria-expanded="false"
        data-target="mainbar"
      >
        <span aria-hidden="true" />
        <span aria-hidden="true" />
        <span aria-hidden="true" />
      </a>
    </div>

    <div id="mainbar" class="navbar-menu" class:is-active={navVisible}>
      <div class="navbar-start">
        <a class="navbar-item" href="/documentation"> Documentation </a>
        <a class="navbar-item" href="/about"> About </a>
        <a class="navbar-item"> Matrix </a>
        <a class="navbar-item" href="http://gitlab.com/Gnurur/chad_launcher/-/issues">
          Report an issue
        </a>
      </div>
      <div class="navbar-end">
        <div class="field">
          <p class="control has-icons-left">
            <input
              bind:value={$query}
              class="has-background-primary-light input is-rounded"
              type="text"
              placeholder="Search..."
            />
            <span class="icon is-small is-left">
              <span class="material-icons">search</span>
            </span>
          </p>
        </div>
      </div>
    </div>
  </nav>
</header>

<style>
</style>

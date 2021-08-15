<script>
  let navVisible = false;
</script>

<header>
  <nav class="navbar" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
      <a class="navbar-item has-text-weight-bold is-size-4" href="/"> GNU/Linux Pirates </a>
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
    </div>
  </nav>
</header>

<style>
</style>

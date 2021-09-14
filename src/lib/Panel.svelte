<script>
  export let title = '';
</script>

<div class="panel">
  <!--
  <button on:click={() => dispatch('close')} icon class="close-button">
    <Icon path={mdiClose} />
  </button>
    -->
  <!--
  <div class="banner">
    <Banner {banner} rounded />
  </div>
    -->
  <div class="title">
    <h1>{title}</h1>
  </div>
  <div class="subtitle">
    <slot name="subtitle" />
  </div>

  <div class="info">
    <slot name="info" />
  </div>

  <div class="text">
    <slot name="text" />
  </div>

  <div class="actions">
    <slot name="actions" />
  </div>
  <slot name="extra" />
</div>

<style>
  .panel {
    padding: 20px;
    display: grid;
    grid-template-columns: 100px auto auto 100px;
    grid-template-rows: 4rem auto 60px;
  }

  .title {
    grid-column: 1 / 4;
  }

  .subtitle {
    grid-column: 4;
  }

  .text {
    grid-column: 1 / 5;
    grid-row: 2;
    overflow-y: scroll;
    height: 60px;
  }

  .actions {
    margin-top: 10px;
    grid-column: 1 / 5;
    grid-row: 3;
  }

  .text {
    font-size: 0.8em;
  }
</style>

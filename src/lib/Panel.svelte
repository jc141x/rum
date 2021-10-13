<script>
  export let title = '';
</script>

<div class="panel">
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
    grid-template-columns: 100px auto auto min-content;
    grid-template-rows: 4rem auto 60px;
  }

  .title {
    grid-column: 1 / 4;
    overflow-x: hidden;
  }
  
  .title h1 {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .subtitle {
    grid-column: 4;
  }

  .text {
    grid-column: 1 / 5;
    grid-row: 2;
    overflow-y: auto;
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

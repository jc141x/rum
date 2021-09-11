<div class="grid full">
  <slot />
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(300px, 100%), 1fr));
    gap: 1rem;
  }

  .full {
    width: 100%;
  }
</style>

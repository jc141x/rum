<script>
  import { cardWidth } from '$lib/store';
</script>

<div class="grid full" style="--cardWidth: {$cardWidth}px">
  <slot />
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(min(var(--cardWidth), 100%), 1fr));
    gap: 1rem;
  }

  .full {
    width: 100%;
  }
</style>

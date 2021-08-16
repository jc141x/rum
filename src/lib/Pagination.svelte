<script>
  import { page, games } from '$lib/store.js';
  $: decrementPage = () => {
    if ($page > 1) {
      $page--;
    }
  };
  $: incrementPage = () => {
    if ($page < Math.ceil($games.length / 20)) {
      $page++;
    }
  };
</script>

<div class="content">
  <nav class="d-flex justify-content-between flex-row" role="navigation" aria-label="pagination">
    <a class="btn" on:click={decrementPage}>Previous</a>
    <a class="btn" on:click={incrementPage}>Next</a>
  </nav>
</div>

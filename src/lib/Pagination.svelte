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

<nav class="pagination is-centered" role="navigation" aria-label="pagination">
  <a class="pagination-previous" on:click={decrementPage}>Previous</a>
  <a class="pagination-next" on:click={incrementPage}>Next page</a>
</nav>

<script>
  import { page } from '$lib/store.js';

  let lastPage = 20; // TODO Figure out amount of pages
</script>

<div class="content">
  <nav>
    <ul class="pagination">
      <!-- Previous page -->
      {#if $page > 1}
        <li class="page-item">
          <a href="#" on:click={() => page.update((p) => p - 1)} class="page-link">
            <i class="fa fa-angle-left" aria-hidden="true" />
            <span class="sr-only">Previous</span>
            <!-- sr-only = only for screen readers -->
          </a>
        </li>
      {/if}
      <!-- Pages and ellipses -->
      {#if $page > 2}
        <li class="page-item">
          <a href="#" on:click={() => page.set(1)} class="page-link">{1}</a>
        </li>
      {/if}
      {#if $page > 3}
        <li class="page-item ellipsis" />
      {/if}
      {#if $page > 1}
        <li class="page-item">
          <a href="#" on:click={() => page.update((p) => p - 1)} class="page-link">{$page - 1}</a>
        </li>
      {/if}
      <!-- Active page item -->
      <li class="page-item active" aria-current="page">
        <a href="#" class="page-link" tabindex="-1">{$page}</a>
      </li>
      {#if $page < lastPage}
        <li class="page-item">
          <a href="#" on:click={() => page.update((p) => p + 1)} class="page-link">{$page + 1}</a>
        </li>
      {/if}
      {#if $page < lastPage - 2}
        <li class="page-item ellipsis" />
      {/if}
      {#if $page < lastPage - 1}
        <li class="page-item">
          <a href="#" on:click={() => page.set(lastPage)} class="page-link">{lastPage}</a>
        </li>
      {/if}
      <!-- Next page -->
      <li class="page-item">
        <a href="#" on:click={() => page.update((p) => p + 1)} class="page-link">
          <i class="fa fa-angle-right" aria-hidden="true" />
          <span class="sr-only">Next</span>
          <!-- sr-only = only for screen readers -->
        </a>
      </li>
    </ul>
  </nav>
</div>

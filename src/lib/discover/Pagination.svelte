<script>
  import { page } from '$lib/store.js';
  import Icon from 'mdi-svelte';
  import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';

  let lastPage = 20; // TODO Figure out amount of pages
</script>

<div class="top">
  <!-- Previous page -->
  <div class="box">
    {#if $page > 1}
      <button on:click={() => page.update((p) => p - 1)}>
        <Icon path={mdiChevronLeft} />
      </button>
    {/if}
  </div>
  <!-- Pages and ellipses -->
  <div class="box">
    {#if $page > 2}
      <button on:click={() => page.set(1)}>
        {1}
      </button>
    {/if}
  </div>
  <div class="box">
    {#if $page > 2}
      <button> ... </button>
    {/if}
  </div>
  <div class="box">
    {#if $page > 1}
      <button on:click={() => page.update((p) => p - 1)}>
        {$page - 1}
      </button>
    {/if}
  </div>
  <div class="box">
    <!-- Active page item -->
    <button class="current">
      {$page}
    </button>
  </div>
  <div class="box">
    {#if $page < lastPage}
      <button on:click={() => page.update((p) => p + 1)}>
        {$page + 1}
      </button>
    {/if}
  </div>
  <div class="box">
    {#if $page < lastPage - 1}
      <button> ... </button>
    {/if}
  </div>
  <div class="box">
    {#if $page < lastPage - 1}
      <button on:click={() => page.set(lastPage)}>
        {lastPage}
      </button>
    {/if}
  </div>
  <div class="box">
    <!-- Next page -->
    <button on:click={() => page.update((p) => p + 1)}>
      <Icon path={mdiChevronRight} />
    </button>
  </div>
</div>

<style>
  .top {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .box {
    width: 50px;
  }

  .current {
    font-size: 1em;
  }
</style>

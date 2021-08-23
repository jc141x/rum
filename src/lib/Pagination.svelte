<script>
  import { page } from '$lib/store.js';
  import { ButtonGroup, ButtonGroupItem, Icon } from 'svelte-materialify/src';
  import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';

  let lastPage = 20; // TODO Figure out amount of pages

</script>

<ButtonGroup>
  <!-- Previous page -->
  {#if $page > 1}
    <ButtonGroupItem on:click={() => page.update((p) => p - 1)}>
        <Icon path={mdiChevronLeft} />
    </ButtonGroupItem>
  {/if}
  <!-- Pages and ellipses -->
  {#if $page > 2}
    <ButtonGroupItem on:click={() => page.set(1)}>
      {1}
    </ButtonGroupItem>
  {/if}
  {#if $page > 3}
    <ButtonGroupItem>
      ...
    </ButtonGroupItem>
  {/if}
  {#if $page > 1}
    <ButtonGroupItem on:click={() => page.update((p) => p - 1)}>
      {$page - 1}
    </ButtonGroupItem>
  {/if}
  <!-- Active page item -->
  <ButtonGroupItem class="primary-color">
    {$page}
  </ButtonGroupItem>
  {#if $page < lastPage}
    <ButtonGroupItem on:click={() => page.update((p) => p + 1)}>
      {$page + 1}
    </ButtonGroupItem>
  {/if}
  {#if $page < lastPage - 2}
    <ButtonGroupItem>
      ...
    </ButtonGroupItem>
  {/if}
  {#if $page < lastPage - 1}
    <ButtonGroupItem on:click={() => page.set(lastPage)}>
      {lastPage}
    </ButtonGroupItem>
  {/if}
  <!-- Next page -->
  <ButtonGroupItem on:click={() => page.update((p) => p + 1)}>
    <Icon path={mdiChevronRight} />
  </ButtonGroupItem>
</ButtonGroup>

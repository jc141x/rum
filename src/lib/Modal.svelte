<script>
  import { fly } from 'svelte/transition';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let title = null;
</script>

<div class="modal" transition:fly={{ y: 50, duration: 200 }} on:click={() => dispatch('close')}>
  <div class="modal-content" on:click={(e) => e.stopPropagation()}>
    {#if title != null}
      <h3>{title}</h3>
    {/if}
    <slot />

    <div class="bottom">
      <slot name="actions" />
    </div>
  </div>
</div>


<style>
  .modal {
    position: fixed;
    z-index: 1;
    left: 0;
    top: 0;
    width: 100%;
    height: 200%;
    top: -50%;
  }

  .modal-content {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 50vw;
    height: 50vh;
    overflow: auto;
  }

  .bottom {
    position: absolute;
    bottom: 20px;
  }
</style>

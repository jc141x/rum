<script>
  import Icon from 'mdi-svelte';
  import { mdiClose } from '@mdi/js';
  import { createEventDispatcher } from 'svelte';
  import Banner from './Banner.svelte';
  const dispatch = createEventDispatcher();

  export let title = '';
  export let banner = '';
</script>

<div class="full">
  <div style="height: 100%; width: 100%; overflow-y: scroll">
    <button on:click={() => dispatch('close')} icon class="close-button">
      <Icon path={mdiClose} />
    </button>
    <Banner {banner} rounded />
    <h6>{title}</h6>
    <slot name="subtitle" />

    <div class="text">
      <slot name="text" />
    </div>

    <slot name="actions" />
  </div>

  <slot name="extra" />
</div>

<style>
  .full {
    height: 100%;
    width: 100%;
    padding: 20px;
  }

  .text {
    font-size: 0.8em;
  }

  .close-button {
    margin-bottom: 20px;
  }
</style>

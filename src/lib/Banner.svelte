<script>
  import { onMount } from 'svelte';
  import { cardTextSize } from '$lib/store';

  export let banner = '';
  export let absolute = false;
  export let rounded = false;
  export let fallbackText = '';

  let loaded = false;
  let image;

  onMount(() => {
    image.onload = () => {
      loaded = true;
    };
  });
</script>

<img src={banner} class:absolute class:rounded alt="banner" class:loaded bind:this={image} />
{#if !loaded && fallbackText != ''}
  <div class="overlay">
    <h5 style="font-size: {$cardTextSize}%">{fallbackText}</h5>
  </div>
{/if}

<style>
  img {
    margin: 0;
    width: 100%;
    opacity: 0;
    transition: opacity 500ms ease-out;
    max-height: 0;
  }

  img.absolute {
    max-width: 100%;
    position: absolute;
    top: 0;
  }

  img.rounded {
    border-radius: 5px;
  }

  img.loaded {
    opacity: 1;
    max-height: 100%;
  }

  .overlay {
    position: absolute;
    top: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>

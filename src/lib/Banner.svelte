<script>
  import defaultBanner from '$lib/assets/default_banner.png';
  import { onMount } from 'svelte';

  export let banner = '';
  export let absolute = false;

  let loaded = false;
  let image;

  onMount(() => {
    image.onload = () => {
      loaded = true;
    };
    image.onerror = () => {
      banner = defaultBanner;
    };
  });
</script>

<img src={banner} class:absolute alt="banner" class:loaded bind:this={image} />

<style>
  img {
    width: 100%;
    opacity: 0;
    transition: opacity 500ms ease-out;
  }

  img.absolute {
    max-width: 100%;
    position: absolute;
    top: 0;
  }

  img.loaded {
    opacity: 1;
  }
</style>

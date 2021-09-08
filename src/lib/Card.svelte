<script>
  import { Card, CardTitle } from 'svelte-materialify/src';
  import defaultBanner from '$lib/assets/default_banner.png';
  import { onMount } from 'svelte';

  export let title = '';
  export let banner = '';
  export let selected = false;
  $: color = selected ? 'primary-color' : '';

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

<div on:click>
  <Card class={color}>
    <div class="img-container">
      <img src={banner} alt="banner" class:loaded bind:this={image} />
    </div>
    <CardTitle>{title}</CardTitle>
  </Card>
</div>

<style>
  .img-container {
    position: relative;
    width: 100%;
    padding-top: 46.74%;
  }
  img {
    max-width: 100%;
    position: absolute;
    top: 0;
    opacity: 0;
    transition: opacity 500ms ease-out;
  }

  img.loaded {
    opacity: 1;
  }
</style>

<script>
  import { Card, CardTitle } from 'svelte-materialify/src';
  import Banner from './Banner.svelte';

  export let title = '';
  export let banner = '';
  export let selected = false;
  $: color = selected ? 'primary-color' : '';
</script>

<div on:click>
  <Card class={color}>
    <div class="img-container">
      <Banner absolute={true} {banner} />
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
</style>

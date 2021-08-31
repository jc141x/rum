<script>
  import { Card, CardTitle } from 'svelte-materialify/src';

  export let title = '';
  export let banner = '';
  export let selected = false;
  $: color = selected ? 'primary-color' : '';
</script>

<div on:click>
  <Card class={color}>
    <div class="img-container">
      <img src={banner} alt="banner" />
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
  }
</style>

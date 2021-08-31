<script>
  import { Card, CardTitle } from 'svelte-materialify/src';

  export let title = '';
  export let banner = '';
  export let selected = false;
  $: color = selected ? 'primary-color' : '';
</script>

<div on:click>
  <Card class={color}>
    <img src={banner} alt="banner" />
    <CardTitle>{title}</CardTitle>
  </Card>
</div>

<style>
  img {
    width: 100%;
    height: 100%;
  }
</style>

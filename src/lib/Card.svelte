<script>
  import { Card, CardTitle } from 'svelte-materialify/src';

  export let title = '';
  export let banner = '';
</script>

<div on:click>
  <Card>
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

<script>
  import { Card, CardTitle } from 'svelte-materialify/src';
  import Banner from './Banner.svelte';

  export let title = '';
  export let banner = '';
  export let selected = false;
</script>

<div on:click class="card-wrapper" class:selected>
  <Card>
    <div class="img-container">
      <Banner absolute={true} rounded {banner} fallbackText={title} />
    </div>
  </Card>
</div>

<style lang="scss">
  @import 'svelte-materialify/src/styles/variables';

  .img-container {
    position: relative;
    width: 100%;
    padding-top: 46.74%;
  }

  .card-wrapper {
    border: 5px solid rgba($color: #000000, $alpha: 0);
    border-radius: 10px;
  }

  .card-wrapper.selected {
    border-color: $primary-color;
  }
</style>

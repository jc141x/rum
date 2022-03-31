<script>
  import Banner from './Banner.svelte';
  import { cardHeight } from '$lib/store';

  export let title = '';
  export let banner = '';
  export let selected = false;
  export let game;
</script>

<div on:click class="col-12" class:selected>
  <div class="card is-center text-center is-pointer is-paddingless">
    <div class="img-container">
      <div>
        <Banner absolute={false} banner={banner} fallbackText={title} {game}/>
      </div>
    </div>
  </div>
</div>

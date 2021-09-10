<script>
  import {
    Card,
    Button,
    Icon,
    CardText,
    CardSubtitle,
    CardTitle,
    CardActions
  } from 'svelte-materialify/src';
  import { mdiClose } from '@mdi/js';
  import { createEventDispatcher } from 'svelte';
  import Banner from './Banner.svelte';
  const dispatch = createEventDispatcher();

  export let title = '';
  export let banner = '';
</script>

<div class="full">
  <Card class="pa-10" style="height: 100%; width: 100%; overflow-y: scroll">
    <Button on:click={() => dispatch('close')} icon class="mb-5">
      <Icon path={mdiClose} />
    </Button>
    <Banner {banner} />
    <CardTitle>{title}</CardTitle>
    <CardSubtitle>
      <slot name="subtitle" />
    </CardSubtitle>

    <CardText>
      <slot name="text" />
    </CardText>

    <CardActions>
      <slot name="actions" />
    </CardActions>
  </Card>

  <slot name="extra" />
</div>

<style>
  .full {
    height: 100%;
    width: 100%;
  }
</style>

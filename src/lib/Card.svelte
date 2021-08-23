<script>
  import {
    Card,
    CardTitle,
    CardSubtitle,
    CardActions,
    Button,
    Row,
    Col, 
CardText,
      Chip} from 'svelte-materialify/src';

  export let title = '';
  export let subtitle = '';
  export let badges = [];
  export let dangerBadges = [];
  export let description = '';
  export let banner = '';
  export let height = 250;
</script>

<div class="card flex-shrink-1 flex-grow-1">
    <Card class="ma-5">
      <img src={banner} alt="banner" />
      <CardTitle><p class="title">{title}</p></CardTitle>
      <CardSubtitle>{subtitle}</CardSubtitle>
      <div class="d-flex flex-row">
          {#each badges as badge}
            <Chip class="ma-1"> {badge} </Chip>
          {/each}
          {#each dangerBadges as badge}
            <Chip class="ma-1 red"> {badge} </Chip>
          {/each}
      </div>
      <CardText><p class="description">{description}</p></CardText>
      <CardActions>
        <slot name="buttons" />
      </CardActions>
    </Card>
</div>

<style>
  .card {
     width: 350px;
  }

  img {
    width: 100%;
    height: 100%;
  }

  .title {
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 1;
    -webkit-box-orient: vertical;
  }

  .description {
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
  }
</style>

    <!--
<div class="card p-0 flex-fill w-250 m-10 d-flex flex-column">
  <img class="img-fluid rounded-top w-full mh-full" src={banner} alt={title} />
  <div class={contentClass}>
    <header class="font-size-18 font-weight-bold">
      {title}
    </header>
    <p class="text-muted my-0">
      {subtitle}
    </p>
    <div class="">
      {#each badges as badge}
        <span class="badge"> {badge} </span>
      {/each}
      {#each dangerBadges as badge}
        <span class="badge badge-danger"> {badge} </span>
      {/each}
    </div>
    <p class="description flex-shrink-1 overflow-hidden">
      {description}
    </p>
    <div class="text-right mt-auto">
      <slot name="buttons" />
    </div>
  </div>
</div>

<style>
  .description {
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
  }
</style>
-->

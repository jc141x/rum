<script>
  export let title = '';
  export let subtitle = '';
  export let badges = [];
  export let dangerBadges = [];
  export let description = '';
  export let banner = '';
  export let height = 250;

  $: contentClass = `content m-15 h-${height} overflow-hidden mh-full d-flex flex-column`
</script>

<div class="card p-0 flex-fill w-250 m-10 d-flex flex-column">
  <!-- Image -->
  <img class="img-fluid rounded-top w-full mh-full" src={banner} alt={title} />
  <div class={contentClass}>
    <!-- Title -->
    <header class="font-size-18 font-weight-bold">
      {title}
    </header>
    <p class="text-muted my-0">
      {subtitle}
    </p>
    <!-- Genres -->
    <div class="">
      {#each badges as badge}
        <span class="badge"> {badge} </span>
      {/each}
      {#each dangerBadges as badge}
        <span class="badge badge-danger"> {badge} </span>
      {/each}
    </div>
    <!-- Description -->
    <p class="description flex-shrink-1 overflow-hidden">
      {description}
    </p>
    <!-- Buttons -->
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

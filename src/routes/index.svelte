<script context="module">
  export const prerender = true;
  export const ssr = false;
</script>

<script>
  import LocalGameCard from '$lib/LocalGameCard.svelte';
  import Card from '$lib/Card.svelte';
  import banner from '$lib/default.png';
</script>

<svelte:head>
  <title>Chad Launcher</title>
</svelte:head>

<div class="d-flex flex-wrap">
  <div class="content">
    <h1 class="content-title">Welcome to Chad Launcher</h1>
  </div>

  <div class="card w-full">
    <h2 class="card-title">Play</h2>
    <div class="d-flex flex-row overflow-x-scroll">
      <Card title="Cool game" {banner} height="100" />
      <Card title="Other game" {banner} height="100" />
      <Card title="Boring game" {banner} height="100" />
      <Card title="Fun game" {banner} height="100" />
      <Card title="Hard game" {banner} height="100" />
    </div>
  </div>

  <div class="card w-full">
    <h2 class="card-title">Discover</h2>
    <p>Popular/trending games here</p>
    <div class="d-flex flex-row overflow-x-scroll">
      <Card title="Popular game" {banner} height="100" />
      <Card title="Other game" {banner} height="100" />
      <Card title="Nice game" {banner} height="100" />
      <Card title="Fun game" {banner} height="100" />
      <Card title="Easy game" {banner} height="100" />
    </div>
  </div>
</div>

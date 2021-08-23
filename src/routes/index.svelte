<script context="module">
  export const prerender = true;
  export const ssr = false;
</script>

<script>
  import LocalGameCard from '$lib/LocalGameCard.svelte';
  import Card from '$lib/Card.svelte';
  import banner from '$lib/default.png';

  import { SlideGroup, SlideItem, Icon } from 'svelte-materialify';
  import { mdiMinus, mdiPlus } from '@mdi/js';
</script>

<svelte:head>
  <title>Chad Launcher</title>
</svelte:head>

<div>
  <div class="content">
    <h6>Welcome to Chad Launcher</h6>
  </div>

  <div>
    <h8>Play</h8>
    <p>Recently played games here</p>
    <SlideGroup activeClass="white-text">
      <span slot="previous">
        <Icon path={mdiMinus} />
      </span>
      <SlideItem>
        <Card title="Cool game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Other game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Boring game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Fun game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Hard game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Dumb game" {banner} height="100" />
      </SlideItem>
      <span slot="next">
        <Icon path={mdiPlus} />
      </span>
    </SlideGroup>
  </div>

  <div>
    <h8>Discover</h8>
    <p>Popular/trending/featured games here</p>
    <SlideGroup activeClass="white-text">
      <span slot="previous">
        <Icon path={mdiMinus} />
      </span>
      <SlideItem>
        <Card title="Cool game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Other game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Boring game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Fun game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Hard game" {banner} height="100" />
      </SlideItem>
      <SlideItem>
        <Card title="Dumb game" {banner} height="100" />
      </SlideItem>
      <span slot="next">
        <Icon path={mdiPlus} />
      </span>
    </SlideGroup>
  </div>

    <br/>
    <br/>
    <br/>
    <br/>
    <br/>
    <br/>
    <br/>
    <br/>
    <br/>
</div>

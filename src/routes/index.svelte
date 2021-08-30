<script context="module">
  export const prerender = true;
  export const ssr = false;
</script>

<script>
  import command from '$lib/command';
  import LocalGameCard from '$lib/LocalGameCard.svelte';
  import Card from '$lib/Card.svelte';
  import banner from '$lib/default.png';

  import { Container, SlideGroup, SlideItem, Icon } from 'svelte-materialify';
  import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';
  const games = ['Cool Game', 'Other Game', 'Boring Game', 'Fun Game', 'Hard Game', 'Dumb Game'];
</script>

<svelte:head>
  <title>Chad Launcher</title>
</svelte:head>

<Container fluid>
  Welcome to Chad Launcher! <br /><br />

  Start by going opening the menu and going to the settings page to configure library paths and
  torrent clients.
  <!--
  <div class="elevation-4 pa-4 rounded-lg">
    <h4 class="text-h4">Play</h4>
    <p class="text-subtitle-1">Recently played games here</p>
    <SlideGroup activeClass="white-text">
      <span slot="previous">
        <Icon path={mdiChevronLeft} />
      </span>
      {#each games as game}
        <SlideItem>
          <div class="ml-2 mr-2">
            <Card title={game} {banner} height="100" />
          </div>
        </SlideItem>
      {/each}
      <span slot="next">
        <Icon path={mdiChevronRight} />
      </span>
    </SlideGroup>
  </div>

  <div class="mt-12 elevation-4 pa-4 rounded-lg">
    <h4 class="text-h4">Discover</h4>
    <p class="text-subtitle-1">Popular/trending/featured games here</p>
    <SlideGroup activeClass="white-text">
      <span slot="previous">
        <Icon path={mdiChevronLeft} />
      </span>
      {#each games as game}
        <SlideItem>
          <div class="ml-2 mr-2">
            <Card title={game} {banner} height="100" />
          </div>
        </SlideItem>
      {/each}
      <span slot="next">
        <Icon path={mdiChevronRight} />
      </span>
    </SlideGroup>
    </div>
    -->
</Container>

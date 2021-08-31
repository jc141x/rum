<script>
  import { genres, selectedGenre, query, page, selectedGame, databaseGames } from '$lib/store.js';
  import GameGrid from '$lib/GameGrid.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import { Container, Select, TextField, Row, Col, ProgressCircular } from 'svelte-materialify/src';
  import GamePanel from '$lib/GamePanel.svelte';
  import { slide, fly, fade } from 'svelte/transition';

  let searchValue = '';

  let genreItems = [];

  genres.subscribe(async ($genres) => {
    genreItems = ['Any'].concat(await $genres);
  });

  let value = 'Any';

  $: {
    $selectedGenre = value !== 'Any' ? value : null;

    if ($page !== 1 && $selectedGenre !== null) {
      page.set(1);
    }
  }
</script>

<svelte:head>
  <title>Chad Launcher - Discover</title>
</svelte:head>

<div class="content ma-5">
  <Row class="pl-5 pr-5">
    <Col>
      <Pagination />
    </Col>
    <Col>
      <Select items={genreItems} bind:value />
    </Col>

    <Col>
      <form
        on:submit={(e) => {
          e.preventDefault();
          $page = 1;
          $query = searchValue;
        }}
      >
        <TextField bind:value={searchValue} placeholder="Search" />
      </form>
    </Col>
  </Row>

  <Row>
    <Col>
      <div
        class="grid full-height pr-3"
        in:fly={{ x: -100, duration: 300, delay: 300 }}
        out:fly={{ x: -100, duration: 300 }}
      >
        <GameGrid />
      </div>
    </Col>
    {#if $selectedGame !== null}
      <Col sm={6} md={4} lg={4}>
        {#await $databaseGames}
          <ProgressCircular indeterminate color="primary" />
        {:then games}
          {#key $selectedGame}
            <div
              in:fly={{ x: 100, duration: 300, delay: 300 }}
              out:fly={{ x: 100, duration: 300 }}
              class="full-height"
            >
              <GamePanel game={games[$selectedGame]} />
            </div>
          {/key}
        {:catch error}
          <p style="color: red">{error.message}</p>
        {/await}
      </Col>
    {/if}
  </Row>
</div>

<style>
  .content {
    max-height: calc(100vh - 100px);
    overflow: hidden;
  }

  .full-height {
    max-height: calc(100vh - 170px);
    min-height: calc(100vh - 170px);
    height: calc(100vh - 170px);
  }

  .grid {
    overflow-y: scroll;
  }
</style>

<script>
  import { genres, selectedGenre, query, page } from '$lib/store.js';
  import GameGrid from '$lib/GameGrid.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import { Container, Select, TextField, Row, Col } from 'svelte-materialify/src';

  let searchValue = '';

  let genreItems = [];

  genres.subscribe(async ($genres) => {
    genreItems = ['Any'].concat(await $genres);
  });

  let value = 'Any';

  $: {
    $selectedGenre = value !== 'Any' ? value : null;
    page.set(1);
  }
</script>

<svelte:head>
  <title>Chad Launcher - Discover</title>
</svelte:head>

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

<Container fluid>
  <GameGrid />
</Container>

<script>
  import { genres, selectedGenre, query, page } from '$lib/store.js';
  import GameGrid from '$lib/GameGrid.svelte';
  import Pagination from '$lib/Pagination.svelte';
  import command from '$lib/command';
  import { Container, Select, TextField, Row, Col } from 'svelte-materialify/src';

  let searchValue = '';

  $: command
    .database('get_genres')
    .then((g) => ($genres = g))
    .catch((err) => console.error(err));

  let value = 'Any';
  let genreItems = [];

  $: {
    if ($genres) {
      genreItems = ['Any'].concat($genres);
    } else {
      genreItems = [];
    }
  }

  $: {
    if (value !== 'Any') {
      $selectedGenre = value;
    } else {
      $selectedGenre = null;
    }
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

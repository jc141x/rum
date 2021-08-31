<script>
  import { localGames, selectedLocalGame } from '$lib/store.js';
  import { Row, Col, ProgressCircular } from 'svelte-materialify';
  import { fly } from 'svelte/transition';
  import Panel from '$lib/library/Panel.svelte';
  import Grid from '$lib/library/Grid.svelte';
</script>

<svelte:head>
  <title>Chad Launcher - Library</title>
</svelte:head>

<div class="content ma-5">
  <Row class="pl-5 pr-5">
    <Col>
      <div
        class="grid full-height pr-3"
        in:fly={{ x: -100, duration: 300, delay: 300 }}
        out:fly={{ x: -100, duration: 300 }}
      >
        <Grid />
      </div>
    </Col>
    {#if $selectedLocalGame !== null}
      <Col sm={6} md={4} lg={4}>
        {#await $localGames}
          <ProgressCircular indeterminate color="primary" />
        {:then games}
          {#key $selectedLocalGame}
            <div
              in:fly={{ x: 100, duration: 300, delay: 300 }}
              out:fly={{ x: 100, duration: 300 }}
              class="full-height"
            >
              <Panel
                game={games[$selectedLocalGame]}
                on:close={() => ($selectedLocalGame = null)}
              />
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
    max-height: calc(100vh - 80px);
    overflow: hidden;
  }

  .full-height {
    max-height: calc(100vh - 130px);
    min-height: calc(100vh - 130px);
    height: calc(100vh - 170px);
  }

  .grid {
    overflow-y: scroll;
  }
</style>

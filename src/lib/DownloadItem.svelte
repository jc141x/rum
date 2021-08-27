<script>
  import {
    Card,
    CardTitle,
    CardSubtitle,
    CardActions,
    Button,
    Icon,
    Row,
    Col,
    CardText,
    Chip,
    ProgressLinear
  } from 'svelte-materialify/src';
  import GameCard from './GameCard.svelte';
  import { mdiPause, mdiPlay } from '@mdi/js';
  import command from '$lib/command';
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let torrent;

  const formatBytes = (bytes, decimals = 2) => {
    if (bytes === 0) return '0 Bytes';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
  };

  let buttonIcon = mdiPlay;

  switch (torrent.state) {
    case 'Paused':
      buttonIcon = mdiPlay;
    case 'Active':
      buttonIcon = mdiPause;
    default:
      buttonIcon = mdiPlay;
  }

  const handleButtonClick = () => {
    dispatch('button-click');
  };
</script>

<Card style="width: 100%">
  <Row class="pl-5 pr-5">
    <Col class="d-flex align-center" sm={3} md={4} lg={4}>
      {torrent.name}
    </Col>
    <Col class="d-flex align-center" sm={2} md={2} lg={1}>
      {formatBytes(torrent.size)}
    </Col>
    <Col class="d-flex align-center">
      <ProgressLinear value={torrent.progress * 100} />
    </Col>
    <Col class="d-flex align-center" sm={2} md={2} lg={1}>
      {formatBytes(torrent.download_speed)}/s
    </Col>
    <Col class="d-flex align-center" sm={2} md={2} lg={1}>
      {formatBytes(torrent.upload_speed)}/s
    </Col>
  </Row>
  <Row class="pl-5 pr-5">
    <Col class="d-flex align-center" sm={3} md={4} lg={4}>
      {torrent.client}
    </Col>
    <Col>
      {torrent.state}
    </Col>
    <Col />
    <Col class="d-flex align-center" sm={1} md={1} lg={1}>
      {#if buttonIcon !== null}
        <Button icon on:click={handleButtonClick}>
          <Icon path={buttonIcon} />
        </Button>
      {/if}
    </Col>
  </Row>
</Card>

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
    Chip,
    ProgressLinear
  } from 'svelte-materialify/src';
  import GameCard from './GameCard.svelte';

  export let torrent;

  const formatBytes = (bytes, decimals = 2) => {
    if (bytes === 0) return '0 Bytes';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
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
    <Col class="d-flex align-center" sm={2} md={2} lg={1}>
      {torrent.state}
    </Col>
  </Row>
</Card>

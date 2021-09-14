<script>
  import Icon from 'mdi-svelte';
  import { mdiPause, mdiPlay } from '@mdi/js';
  import { createEventDispatcher } from 'svelte';
  import ProgressBar from '@okrad/svelte-progressbar';
  import { styles } from '$lib/styles';
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

  let buttonIcon = null;

  $: {
    switch (torrent.state) {
      case 'Paused':
        buttonIcon = mdiPlay;
        break;
      case 'Active':
        buttonIcon = mdiPause;
        break;
      default:
        buttonIcon = mdiPlay;
        break;
    }
  }

  const handleButtonClick = () => {
    dispatch('toggle-pause');
  };
</script>

<div class="top">
  <div class="col">
    {torrent.name}
  </div>
  <div class="col">
    {formatBytes(torrent.size)}
  </div>
  <div class="col progress">
    <!--<BarLoader size="60" color={$styles.primary} unit="px" duration="1s" />-->
    <ProgressBar
      style={'thin'}
      series={[{ perc: torrent.progress * 100, color: $styles.primary }]}
      width={'300'}
    />
  </div>
  <div class="col">
    {(torrent.progress * 100).toFixed(2)}%
  </div>
  <div class="col">
    {formatBytes(torrent.download_speed)}/s
  </div>
  <div class="col">
    {formatBytes(torrent.upload_speed)}/s
  </div>
  <div class="col">
    {torrent.client}
  </div>
  <div class="col">
    {torrent.state}
  </div>
  <div />
  <div class="col">
    {#if buttonIcon !== null}
      <button on:click={handleButtonClick}>
        <Icon path={buttonIcon} />
      </button>
    {/if}
  </div>
</div>

<style>
  .top {
    display: grid;
    background-color: var(--secondary);
    grid-template-columns: 3fr 1fr 4fr 1fr 1fr 1fr;
    margin: 20px;
    padding: 20px;
    border-radius: 10px;
  }

  .progress {
    width: 100%;
  }
</style>

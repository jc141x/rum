<script>
  import { selectedGame } from '$lib/store.js';
  import banner from './default.png';
  const trackers = [
    'udp://tracker.leechers-paradise.org:6969/announce',
    'udp://tracker.opentrackr.org:1337/announce',
    'udp://tracker.zer0day.to:1337/announce',
    'udp://eddie4.nl:6969/announce',
    'udp://46.148.18.250:2710',
    'udp://opentor.org:2710',
    'http://tracker.dler.org:6969/announce',
    'udp://9.rarbg.me:2730/announce',
    'udp://9.rarbg.to:2770/announce',
    'udp://tracker.pirateparty.gr:6969/announce',
    'http://retracker.local/announce',
    'http://retracker.ip.ncnet.ru/announce',
    'udp://exodus.desync.com:6969/announce',
    'udp://ipv4.tracker.harry.lu:80/announce',
    'udp://open.stealth.si:80/announce',
    'udp://coppersurfer.tk:6969/announce'
  ];
  export let game;
  function get_banner(game) {
    return game.banner_path === null
      ? banner
      : `https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/${game.banner_path}`;
  }
  function getMagnet(game) {
    let magnet = `magnet:?xt=urn:btih:${game.hash}&dn=${game.name}`;
    for (let tracker of trackers) {
      magnet += `&tr=${tracker}`;
    }
    return magnet;
  }
  function truncateString(str, n) {
    if (str.length > n) {
      return str.substring(0, n) + '...';
    } else {
      return str;
    }
  }
  function toTitleCase(str) {
    return str.replace(/\w\S*/g, function (txt) {
      return txt.charAt(0).toUpperCase() + txt.substr(1).toLowerCase();
    });
  }
</script>

<div class="card p-0">
  <!-- Image -->
  <img class="img-fluid rounded-top" src={get_banner(game)} alt={game.name} />
  <div class="content m-15">
    <!-- Title -->
    <header class="font-size-18 font-weight-bold">
      {game.name}
    </header>
    <p class="text-muted my-0">
      {toTitleCase(game.type)}
    </p>
    <!-- Genres -->
    {#each game.genres as genre}
        <span class="badge"> {genre} </span>
    {/each}
    {#if game.nsfw}
      <span class="badge badge-danger"> 18+ </span>
    {/if}
    <!-- Description -->
    <p class="">
      {truncateString(game.description, 250)}
    </p>
    <!-- Download -->
    <div class="text-right">
      <a href={getMagnet(game)} class="btn" target="_blank">Download</a>
    </div>
  </div>
</div>

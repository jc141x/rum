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
    return game.banner_rel_path === null
      ? banner
      : `https://gitlab.com/chad-productions/chad_launcher_banners/-/raw/master/${game.banner_rel_path}`;
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
</script>

<div class="tile is-parent is-4">
  <div class="tile is-child">
    <a href={getMagnet(game)}>
      <div class="game-card card is-clickable has-background-primary:hover">
        <div class="card-image">
          <figure class="image">
            <img src={get_banner(game)} alt={game.name} />
          </figure>
        </div>
        <div class="card-content">
          <p class="title is-4">{game.name}</p>
          <p class="subtitle is-7 has-text-grey">{game.hash}</p>
          <div class="tags">
            {#if game.genres != ''}
              {#each game.genres.split(';') as tag}
                <span class="tag is-primary is-light">{tag}</span>
              {/each}
            {/if}
            {#if game.nsfw}
              <span class="tag is-danger is-light">NSFW</span>
            {/if}
          </div>

          <div class="content">
            <p class=" has-text-grey-dark">
              {truncateString(game.description, 300)}
            </p>
          </div>
        </div>
      </div>
    </a>
  </div>
</div>

<style>
</style>

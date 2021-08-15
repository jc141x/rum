<script>
  import { games } from '$lib/store.js';
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

  function getMagnet(game) {
    let magnet = `magnet:?xt=urn:btih:${game.hash}&dn=${game.name}`;
    for (let tracker of trackers) {
      magnet += `&tr=${tracker}`;
    }
    return magnet;
  }
</script>

{#each $games as game}
  <div class="box">
    <div class="level">
      <div class="level-left">
        <p class="mr-3">{game.name}</p>
        <div class="tags">
          {#if game.genres != null}
            {#each game.genres.split(';') as tag}
              <span class="tag is-primary is-light">{tag}</span>
            {/each}
          {/if}
          {#if game.nsfw}
            <span class="tag is-danger is-light">NSFW</span>
          {/if}
        </div>
      </div>
      <div class="level-right">
        <a href={getMagnet(game)}>
          <span class="material-icons icon">download</span>
        </a>
      </div>
    </div>
  </div>
{/each}

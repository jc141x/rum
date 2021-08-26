<script context="module">
  export const prerender = true;
  export const ssr = false;
</script>

<script>
  import LocalGameCard from '$lib/LocalGameCard.svelte';
  import Card from '$lib/Card.svelte';
  import banner from '$lib/default.png';

  import { Container, SlideGroup, SlideItem, Icon } from 'svelte-materialify';
  import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';
  const games = ['Cool Game', 'Other Game', 'Boring Game', 'Fun Game', 'Hard Game', 'Dumb Game'];
  import { SlideGroup, SlideItem, Icon } from 'svelte-materialify';
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';

  const test_downloads = async () => {
    await invoke('init_download_clients');

    await invoke('add_qbittorrent_client', {
      name: 'NewQBittorrent',
      options: {
        host: 'http://localhost:8080',
        username: 'admin',
        password: 'adminadmin'
      }
    });

    let clients = await invoke('list_clients');

    for (const client of clients) {
      console.log(client);
      const list = await invoke('list_downloads', { client });
      console.log(list);

      /*
      if (client == 'Deluge') {
        let torrent = list.filter((t) => t.name == 'Arch Linux')[0];
        await invoke('resume_download', { client, torrentId: torrent.id });
      }
        */
    }
  };

  test_downloads().catch((err) => console.error(err));
</script>

<svelte:head>
  <title>Chad Launcher</title>
</svelte:head>

<Container fluid>
  <div class="elevation-4 pa-4 rounded-lg">
    <h4 class="text-h4">Play</h4>
    <p class="text-subtitle-1">Recently played games here</p>
    <SlideGroup activeClass="white-text">
      <span slot="previous">
        <Icon path={mdiChevronLeft} />
      </span>
      {#each games as game}
        <SlideItem>
          <div class="ml-2 mr-2">
            <Card title={game} {banner} height="100" />
          </div>
        </SlideItem>
      {/each}
      <span slot="next">
        <Icon path={mdiChevronRight} />
      </span>
    </SlideGroup>
  </div>

  <div class="mt-12 elevation-4 pa-4 rounded-lg">
    <h4 class="text-h4">Discover</h4>
    <p class="text-subtitle-1">Popular/trending/featured games here</p>
    <SlideGroup activeClass="white-text">
      <span slot="previous">
        <Icon path={mdiChevronLeft} />
      </span>
      {#each games as game}
        <SlideItem>
          <div class="ml-2 mr-2">
            <Card title={game} {banner} height="100" />
          </div>
        </SlideItem>
      {/each}
      <span slot="next">
        <Icon path={mdiChevronRight} />
      </span>
    </SlideGroup>
  </div>
</Container>

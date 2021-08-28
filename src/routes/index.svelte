<script context="module">
  export const prerender = true;
  export const ssr = false;
</script>

<script>
  import command from '$lib/command';
  import LocalGameCard from '$lib/LocalGameCard.svelte';
  import Card from '$lib/Card.svelte';
  import banner from '$lib/default.png';

  import { Container, SlideGroup, SlideItem, Icon } from 'svelte-materialify';
  import { mdiChevronLeft, mdiChevronRight } from '@mdi/js';
  const games = ['Cool Game', 'Other Game', 'Boring Game', 'Fun Game', 'Hard Game', 'Dumb Game'];

  const test_downloads = async () => {
    // Reconnects to previously added torrent clients
    await command.download('init_clients');

    // Adding a new QBittorrent client is straightforward
    await command.download('add_qbittorrent_client', {
      name: 'NewQBittorrent',
      options: {
        host: 'http://localhost:8080',
        username: 'admin',
        password: 'adminadmin'
      }
    });

    // But adding a Deluge client is more complicated
    // First we create a connection to the Web API
    await command.download('create_deluge_client', {
      options: {
        web_address: 'http://localhost:8112/json',
        web_password: 'deluge'
      }
    });

    // Now we can list the hosts and choose one
    let hosts = await command.download('list_deluge_hosts');
    console.log(hosts);
    let daemonId = hosts.filter((h) => h.host == 'localhost')[0].id;

    // Finally we connect to the chosen daemon
    // This will also store the connection in the config file
    await command.download('deluge_connect_daemon', {
      name: 'NewDeluge',
      daemonId
    });

    // We can list the connected torrent clients
    let clients = await command.download('list_clients');
    console.log(clients);

    const list = await command.download('list_all_downloads');
    console.log(list);

    /*
    for (const client of clients) {
      console.log(client);
      // And list downloads with label/catergory "chad" for each client
      const list = await command.download('list_downloads', { client });
      console.log(list);

      if (client == 'Deluge') {
        let torrent = list.filter((t) => t.name == 'Arch Linux')[0];
        await invoke('resume_download', { client, torrentId: torrent.id });
      }
    }
      */
  };

  //test_downloads().catch((err) => console.error(err));
</script>

<svelte:head>
  <title>Chad Launcher</title>
</svelte:head>

<Container fluid>
  Welcome to Chad Launcher! <br /><br />

  Start by going opening the menu and going to the settings page to configure library paths and
  torrent clients.
  <!--
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
    -->
</Container>

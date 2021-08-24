<script>
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import { page, localGames, config } from '$lib/store.js';
  import LocalGameCard from './LocalGameCard.svelte';

  $: {
      if ($config) {
          console.log("update library");
      invoke('reload_local_games');
      invoke('get_local_games')
              .then((g) => {

                  $localGames = g
                  console.log(g);
              })
        .catch((err) => console.error(err));
      }
  }
</script>

<div class="d-flex flex-row flex-wrap">
  {#each $localGames as game (game.id)}
    <LocalGameCard {game} />
  {/each}
</div>

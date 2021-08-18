<script>
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import Card from './Card.svelte';
  import banner from './default.png';

  export let game;

  $: banner_src =
    game.banner === null
      ? banner
      : game.banner;
</script>

<Card
  title={game.name}
  banner={banner_src}
  height={150}
>
  <button slot="buttons" class="btn" target="_blank">
    Launch
  </button>
</Card>

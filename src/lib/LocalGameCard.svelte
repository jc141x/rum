<script>
  import { invoke } from '../../node_modules/@tauri-apps/api/tauri';
  import Card from './Card.svelte';
  import banner from './default.png';
  import { Button, Menu, List, ListItem } from 'svelte-materialify/src';

  export let game;

  $: banner_src = game.banner === null ? banner : game.banner;

  const handleLaunch = (script) => {
    invoke('run_game', { index: game.id, script });
  };

</script>

<Card title={game.name} banner={banner_src} height={150}>
  <div slot="buttons">
    <Menu>
      <div slot="activator">
        <Button>Launch</Button>
      </div>
      <List>
        {#each game.scripts as script}
          <ListItem on:click={() => handleLaunch(script)}>
            {script}
          </ListItem>
        {/each}
      </List>
    </Menu>
  </div>
</Card>

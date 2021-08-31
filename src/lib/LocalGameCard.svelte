<script>
  import command from '$lib/command';
  import Card from './Card.svelte';
  import banner from './default.png';
  import { Button, Menu, List, ListItem } from 'svelte-materialify/src';
  import GameCard from './GameCard.svelte';

  export let game;
  export let selected;

  $: banner_src = game.banner === null ? banner : game.banner;

  const handleLaunch = (script) => {
    command.library('run_game', { index: game.id, script });
  };
</script>

<Card title={game.name} banner={banner_src} {selected} on:click>
  <div slot="buttons">
    {#if game.scripts.length > 1}
      <Menu>
        <div slot="activator">
          <Button>Launch</Button>
        </div>
        <List>
          {#each game.scripts as script}
            <ListItem on:click={() => handleLaunch(script.script)}>
              {script.name}
            </ListItem>
          {/each}
        </List>
      </Menu>
    {:else if game.scripts.length > 0}
      <Button on:click={() => handleLaunch(game.scripts[0].script)}>Launch</Button>
    {/if}
  </div>
</Card>

<script>
  import { onMount } from 'svelte';
  import { TextField, AppBar, Button, Icon } from 'svelte-materialify';
  import { mdiMenu, mdiMagnify } from '@mdi/js';
  import { query, sidebarActive } from './store.js';
  import { fly } from 'svelte/transition';
  let searchActive;
  onMount(() => {
    searchActive = false;
  });

  const search = (e) => {
    if (e.key === 'Enter') {
      searchActive = false;
      console.log('hi');
    }
  };
</script>

<AppBar class="grey darken-4">
  <div slot="icon">
    <Button
      fab
      depressed
      on:click={() => {
        $sidebarActive = true;
      }}
    >
      <Icon path={mdiMenu} />
    </Button>
  </div>
  <span slot="title">Chad Launcher</span>
  <div class="flex-grow-1" />
  {#if searchActive}
    <div transition:fly={{ y: 50, duration: 200 }}>
      <TextField on:keyup={search} bind:value={$query} style="max-width:300px" dense rounded filled
        >Search</TextField
      >
    </div>
  {:else}
    <div transition:fly={{ y: 50, duration: 200 }}>
      <Button fab depressed on:click={() => (searchActive = true)}>
        <Icon path={mdiMagnify} />
      </Button>
    </div>
  {/if}
</AppBar>

<script>
  import {
    Overlay,
    NavigationDrawer,
    List,
    ListItem,
    Subheader,
    ListItemGroup,
    Icon
  } from 'svelte-materialify';
  import { mdiLibrary, mdiCompass, mdiGavel, mdiHome } from '@mdi/js';
  import { sidebarActive as active } from './store.js';
</script>

<div class="sidebar" class:active={$active}>
  <NavigationDrawer fixed active={$active} class="pt-16 elevation-4">
    <List nav dense>
      <a href="/" style="text-decoration: none" on:click={() => ($active = false)}>
        <ListItem>
          <span slot="prepend">
            <Icon path={mdiHome} />
          </span>
          Index
        </ListItem>
      </a>
      <ListItemGroup>
        <a href="/library" style="text-decoration: none" on:click={() => ($active = false)}>
          <ListItem>
            <span slot="prepend">
              <Icon path={mdiLibrary} />
            </span>
            Library
          </ListItem>
        </a>
        <a href="/discover" style="text-decoration: none" on:click={() => ($active = false)}>
          <ListItem>
            <span slot="prepend">
              <Icon path={mdiCompass} />
            </span>
            Discover
          </ListItem>
        </a>
        <a href="/settings" style="text-decoration: none" on:click={() => ($active = false)}>
          <ListItem>
            <span slot="prepend">
              <Icon path={mdiGavel} />
            </span>
            Settings
          </ListItem>
        </a>
      </ListItemGroup>
    </List>
  </NavigationDrawer>
</div>

<Overlay
  active={$active}
  index={1}
  on:click={() => {
    $active = false;
  }}
/>

<style>
  .sidebar.active {
    z-index: 5;
  }
</style>

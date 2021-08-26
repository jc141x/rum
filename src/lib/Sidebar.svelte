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

<div class="sidebar">
  <NavigationDrawer active={$active} class="elevation-4" style="z-index:5">
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
  .sidebar {
    z-index: 5;
    position: fixed;
    height: 100%;
  }
</style>

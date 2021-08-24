<script>
    import { Tabs, Tab, AppBar, ButtonGroup, ButtonGroupItem } from 'svelte-materialify/src';

    export let value = 0;
</script>

<div class="header">
    <!--
<AppBar dense>
  <span slot="title"> Chad Launcher </span>
  <div slot="extension">
    <Tabs bind:value fixedTabs>
      <div slot="tabs">
        <Tab>Home</Tab>
        <Tab>Library</Tab>
        <Tab>Discover</Tab>
        <Tab>Documentation</Tab>
      </div>
    </Tabs>
  </div>
</AppBar>
    -->
    <Tabs bind:value fixedTabs>
      <div slot="tabs">
        <Tab>Home</Tab>
        <Tab>Library</Tab>
        <Tab>Discover</Tab>
        <Tab>Information</Tab>
        <Tab>Settings</Tab>
      </div>
    </Tabs>
    </div>

<style>
    .header {
        position: sticky;
        top: 0;
        width: 100%;
        z-index: 100;
    }
</style>

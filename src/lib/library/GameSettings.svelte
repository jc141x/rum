<script>
  import command from '$lib/command';
  import Modal from '$lib/Modal.svelte';
  import { open } from '../../../node_modules/@tauri-apps/api/dialog';
  import { allfiles } from '$lib/store';

  export let game;

  const setBanner = async () => {
    let banner = await open({
      filters: $allfiles
        ? []
        : [
            {
              name: 'Picture',
              extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp']
            }
          ]
    });
    if (banner) {
      await command.library('set_banner', { index: game.id, path: banner });
    }
  };
  const unsetBanner = async () => {
    await command.library('remove_banner', { index: game.id });
  };
</script>

<Modal on:close title={game.name}>
  <div class="controls">
    <p>Changes will be visible next time you start the app</p>
    <button on:click={setBanner}>Set Banner</button>
    <button on:click={unsetBanner}>Delete banner</button>
  </div>
</Modal>

<style>
</style>

<script>
  import command from '$lib/command';
  import Modal from '$lib/Modal.svelte';
  import { open } from '../../../node_modules/@tauri-apps/api/dialog';

  export let game;

  const setBanner = async () => {
    let banner = await open({
      filters: [
        {
          name: 'Picture',
          extensions: ['png', 'jpg', 'jpeg'],
        },
      ]
    });
    if (banner) {
      await command.library('set_banner', { index: game.id, path: banner });
    }
  };
  const unsetBanner = async () => {
    await command.library('remove_banner', { index: game.id });
  }

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
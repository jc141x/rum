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
      window.location.reload(true); // there might be a better way to achieve this, needed for displaying the banner
    }
  };
  const unsetBanner = async () => {
    await command.library('remove_banner', { index: game.id });
    window.location.reload(true);
  }

</script>

<Modal on:close title={game.name}>
  <div class="controls">
    <button on:click={setBanner}>Set Banner</button>
    <button on:click={unsetBanner}>Delete banner</button>
  </div>
</Modal>

<style>

</style>
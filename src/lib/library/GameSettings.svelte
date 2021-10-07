<script>
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
      // TODO: set banner
    }
  }

</script>

<Modal on:close title={game.name}>
  <div class="controls">
    <button on:click={setBanner}>Set Banner</button>
  </div>
</Modal>

<style>

</style>
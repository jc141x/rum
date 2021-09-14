<script>
  import { createEventDispatcher } from 'svelte';
  import ConfigureDelugeModal from './ConfigureDelugeModal.svelte';
  import ConfigureQBittorrentModal from './ConfigureQBittorrentModal.svelte';
  import Modal from '$lib/Modal.svelte';
  import { fly } from 'svelte/transition';

  export let active;

  const dispatch = createEventDispatcher();

  const backends = ['Deluge', 'qBittorrent'];

  let state = 'select_backend';
  let selectedBackend = null;

  let clientName = '';

  const handleSelectBackend = (backend) => {
    selectedBackend = backend;
    clientName = selectedBackend;
    state = 'configure_backend';
  };
</script>

{#if active}
  <Modal on:close>
    {#if state == 'select_backend'}
      <div in:fly={{ x: 100, duration: 200, delay: 200 }} out:fly={{ x: -100, duration: 200 }}>
        <h3>Choose a torrent client</h3>
        <div>
          {#each backends as backend}
            <div>
              <button on:click={() => handleSelectBackend(backend)}>{backend}</button>
            </div>
          {/each}
        </div>
        <div class="bottom">
          <button on:click={() => dispatch('close')}>Cancel</button>
        </div>
      </div>
    {:else if state == 'configure_backend'}
      <div in:fly={{ x: 100, duration: 200, delay: 200 }} out:fly={{ x: -100, duration: 200 }}>
        {#if selectedBackend == 'qBittorrent'}
          <ConfigureQBittorrentModal
            bind:clientName
            on:close
            on:done={() => (state = 'done')}
            on:back={() => (state = 'select_backend')}
          />
        {:else if selectedBackend == 'Deluge'}
          <ConfigureDelugeModal
            bind:clientName
            on:close
            on:done={() => (state = 'done')}
            on:back={() => (state = 'select_backend')}
          />
        {:else}
          Not implemented
        {/if}
      </div>
    {:else if state == 'done'}
      <div in:fly={{ x: 100, duration: 200, delay: 200 }} out:fly={{ x: -100, duration: 200 }}>
        <h3>Client "{clientName}" added successfully.</h3>
      </div>
      <div class="bottom">
        <button
          on:click={() => {
            state = 'select_backend';
            dispatch('close');
          }}>Close</button
        >
      </div>
    {/if}
  </Modal>
{/if}

<style>
  .bottom {
    position: absolute;
    bottom: 20px;
  }
</style>

<script>
  import { styles } from '$lib/styles';
  import { Pulse } from 'svelte-loading-spinners';
  import command from '$lib/command';
  import { torrentClients } from '$lib/store';
  import { createEventDispatcher, onMount } from 'svelte';
  import Icon from 'mdi-svelte';
  import { fly } from 'svelte/transition';
  import Modal from '$lib/Modal.svelte';
  import { mdiFolder } from '@mdi/js';
  import { open } from '../../../node_modules/@tauri-apps/api/dialog';
  const dispatch = createEventDispatcher();

  export let game;
  export let active;

  let selectedClient = null;
  let savePath = '';
  let doneMessage = null;

  const handleDownload = () => {
    command
      .download('add_game', {
        client: selectedClient,
        game,
        options: {
          name: game.name,
          save_path: savePath
        }
      })
      .then(() => (doneMessage = 'Success!'))
      .catch((e) => (doneMessage = `Failed to add download: ${e.message}`));
  };

  const selectSavePath = async () => {
    savePath = await open({
      defaultPath: savePath || null,
      directory: true
    });
  };
</script>

{#if active}
  <Modal on:close>
    {#if doneMessage === null}
      {#await $torrentClients}
        <Pulse size="60" color={$styles.primary} unit="px" duration="1s" />
      {:then clients}
        <h3>Download {game.name}</h3>
        <div class="content">
          <p>Torrent Client</p>
          <select bind:value={selectedClient}>
            {#each clients as client}
              <option value={client}>{client}</option>
            {/each}
          </select>
          <div />
          <p>Save path</p>
          <input bind:value={savePath} />
          <div>
            <button on:click={selectSavePath}><Icon path={mdiFolder} /></button>
          </div>
        </div>
        <div class="bottom">
          <button on:click={() => dispatch('close')}>Close</button>
          <button on:click={handleDownload}>Download</button>
        </div>
      {:catch error}
        <p style="color: red">{error.message}</p>
      {/await}
    {:else}
      <div transition:fly={{ x: 100, duration: 200 }}>
        <h3>Download {game.name}</h3>
        <div>
          {doneMessage}
        </div>
        <div class="bottom">
          <button
            on:click={() => {
              doneMessage = null;
              dispatch('close');
            }}
          >
            Close
          </button>
        </div>
      </div>
    {/if}
  </Modal>
{/if}

<style>
  .content {
    display: grid;
    grid-template-columns: max-content auto max-content;
    grid-gap: 10px;
    align-items: center;
    margin-bottom: 20px;
    margin-top: 20px;
  }

  .bottom {
    position: absolute;
    bottom: 20px;
  }
</style>

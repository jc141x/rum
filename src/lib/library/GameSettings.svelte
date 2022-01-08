<script>
  import command from '$lib/command';
  import Modal from '$lib/Modal.svelte';
  import { open } from '../../../node_modules/@tauri-apps/api/dialog';
  import { allfiles } from '$lib/store';
  import { onMount } from 'svelte';
  import Icon from 'mdi-svelte';
  import { mdiContentSave } from '@mdi/js';

  export let game;
  let wrapper;
  let env;
  let args;

  onMount(async () => {
    let conf = {}
    command.library('read_game_config', { index: game.id }).then((res) => {
      conf = JSON.parse(res);
      wrapper = conf?.wrapper;
      env = conf?.env?.join("\n");
      args = conf?.args;
    }).catch((err) => {
      console.warn(err);
    });
  });
  
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
      command.library('set_banner', { index: game.id, path: banner })
      .catch((err) => {
        console.warn(err);
      }).finally(() => {
        document.activeElement.blur();
      });
    }
  };
  const unsetBanner = async () => {
    command.library('remove_banner', { index: game.id })
    .catch((err) => {
      console.warn(err);
    }).finally(() => {
      document.activeElement.blur();
    });
  };

  const saveConfig = async () => {

    await command.library(
      'save_game_config',
      { index: game.id, wrapper: wrapper, env: env.split("\n"), args: args }
    );
    document.activeElement.blur();
  };
</script>

<Modal on:close title={game.name}>
  <div class="controls">
    <h4>Banner</h4>
    <p>Changes will be visible next time you start the app</p>
    <button on:click={setBanner}>Set Banner</button>
    <button on:click={unsetBanner}>Delete banner</button>
  </div>
  <div class="controls">
    <h4>Launch Options</h4>
    <p>
      <button on:click={saveConfig}><Icon path={mdiContentSave} /> Save options</button>
    </p>
    <label for="wrapper">Wrapper</label>
    <input id="wrapper"bind:value={wrapper}/>
    <label for="env">Environment</label>
    <br>
    <small>One definition per line</small>
    <textarea id="env" bind:value={env}/>
    <label for="args">Arguments</label>
    <input id="args" bind:value={args}/>
  </div>
</Modal>

<style>
.controls input, textarea {
  margin-bottom: 1rem;
}
</style>

import command from '$lib/command';
import { writable, derived } from 'svelte/store';

function configStore() {
  const store = writable({});

  const load = async () => {
    store.set(await command.config('get'));
  };

  const setup = async () => {
    await load();
    store.subscribe(async (config) => {
      if (config != null && config != {}) {
        await command.config('set', { newConfig: config });
        await command.config('save');
      }
    });
  };

  return {
    ...store,
    load,
    setup
  };
}

export const config = configStore();
export const games = writable([]);
export const localGames = writable([]);
export const genres = writable([]);
export const selectedGame = writable(0);
export const selectedGenre = writable('');
export const page = writable(1);
export const mode = writable('grid');
export const query = writable('');
export const sidebarActive = writable(false);

// Called when the window has loaded
// Tauri commands can only be invoked after the window has loaded
export const load = async () => {
  await config.setup();
};

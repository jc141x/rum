import command from '$lib/command';
import { writable, derived } from 'svelte/store';
import { asyncable } from 'svelte-asyncable';
import { writable as storable } from 'svelte-local-storage-store';

// Creates an asyncable store with a "reload trigger".
// This reload trigger makes sure the getter gets called and triggers subscriptions
const asyncableReload = (getter, setter, stores = []) => {
  const reloadTrigger = writable(false);
  const store = asyncable(getter, setter, [...stores, reloadTrigger]);

  const reload = () => reloadTrigger.update((value) => !value);

  return {
    ...store,
    reload
  };
};

export const config = asyncableReload(
  async () => await command.config('get'),
  async (config) => {
    await command.config('set', { newConfig: config });
    await command.config('save');
    // TODO: Maybe the backend should take care of this automatically?
    await command.library('reload_games');
  }
);

export const query = writable('');

export const localGames = asyncable(async () => await command.library('get_games'), null);

export const selectedLocalGame = writable(null);
export const selectedGenre = writable(null);

export const mode = writable('grid');
export const sidebarActive = writable(false);

export const decorations = storable('decorations', 'right');
export const cardWidth = storable('cardWidth', 300);
export const cardHeight = storable('cardHeight', "43/92");
export const cardTextSize = storable('cardTextSize', 100);

export const allfiles = storable('allfiles', false);

export const load = async () => {
  await command.library('reload_games');
};

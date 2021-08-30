import command from '$lib/command';
import { writable, derived } from 'svelte/store';
import { asyncable } from 'svelte-asyncable';

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

export const localGames = asyncable(async () => await command.library('get_games'), null);

export const selectedGenre = writable('');
export const page = writable(1);
export const query = writable('');

export const databaseGames = asyncable(
  async ($selectedGenre, $page, $query) => {
    let opts = { page_number: $page - 1, page_size: 20 };

    if ($query != '') {
      opts.search = $query;
    }

    if ($selectedGenre != '') {
      opts.filter_genre = $selectedGenre;
    }

    return await command.database('get_games', { opts });
  },
  null,
  [selectedGenre, page, query]
);

export const genres = asyncable(async () => await command.database('get_genres'), null);
export const downloads = asyncableReload(
  async () => await command.download('list_all_downloads'),
  null
);

export const torrentClients = asyncable(async () => {
  await command.download('init_clients');
  return await command.download('list_clients');
}, null);

export const mode = writable('grid');
export const selectedGame = writable(null);
export const sidebarActive = writable(false);

export const load = async () => {
  await Promise.all([command.library('reload_games'), command.download('init_clients')]);
};

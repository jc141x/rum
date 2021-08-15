import { writable, derived } from 'svelte/store';

export const games = writable([]);
export const genres = writable([]);
export const selectedGame = writable(0);
export const selectedGenre = writable(0);
export const page = writable(1);
export const mode = writable('grid');
export const query = writable('');
export const filteredGames = derived([games, query], ([$games, $query], set) => {
  const regex = new RegExp(`.*${$query}.*`, 'i');
  set($games.filter((game) => regex.test(game.name)));
});

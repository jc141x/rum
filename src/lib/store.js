import { writable, derived } from 'svelte/store';

export const config = writable(null);
export const games = writable([]);
export const localGames = writable([]);
export const genres = writable([]);
export const selectedGame = writable(0);
export const selectedGenre = writable('');
export const page = writable(1);
export const mode = writable('grid');
export const query = writable('');
export const sidebarActive = writable(false);

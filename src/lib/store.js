import { writable } from 'svelte/store';

export const games = writable([]);
export const genres = writable([]);
export const selectedGame = writable(0);
export const selectedGenre = writable(0);
export const page = writable(1);

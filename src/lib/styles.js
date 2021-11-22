import { derived } from 'svelte/store';
import { writable } from 'svelte-local-storage-store';

export const defaultStyles = {
  'bg-bottom': '#0E0E0E',
  'bg-top': '#292929',
  primary: '#c2c2c2',
  secondary: '#D63600;'
};

export const styles = writable('theme', defaultStyles);

export const cssStyles = derived(styles, ($styles) => {
  return Object.entries($styles)
    .map(([key, value]) => `--${key}:${value}`)
    .join(';');
});

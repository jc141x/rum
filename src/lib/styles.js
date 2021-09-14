import { derived } from 'svelte/store';
import { writable } from 'svelte-local-storage-store';

export const defaultStyles = {
  'bg-bottom': '#0A0E1E',
  'bg-top': '#171D35',
  primary: '#317797',
  secondary: '#1E2543'
};

export const styles = writable('theme', defaultStyles);

export const cssStyles = derived(styles, ($styles) => {
  console.log('update styles');
  return Object.entries($styles)
    .map(([key, value]) => `--${key}:${value}`)
    .join(';');
});

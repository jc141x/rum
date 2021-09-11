import { writable, derived } from 'svelte/store';

export const styles = writable({
  'bg-bottom': '#0A0E1E',
  'bg-top': '#171D35',
  primary: '#317797',
  secondary: '#1E2543'
});

export const cssStyles = derived(styles, ($styles) =>
  Object.entries($styles)
    .map(([key, value]) => `--${key}:${value}`)
    .join(';')
);

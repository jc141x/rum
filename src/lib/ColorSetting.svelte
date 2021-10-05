<script>
  import { styles } from '$lib/styles';

  export let key;
</script>

<input type="color" bind:value={$styles[key]} />

<style>
  input {
    appearance: menulist-button;
    border: none;
    height: 1.5rem;
    padding:0;
  }
</style>

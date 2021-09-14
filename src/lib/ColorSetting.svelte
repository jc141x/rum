<script>
  import { styles } from '$lib/styles';

  export let key;
</script>

<input type="color" bind:value={$styles[key]} />

<style>
  input {
    background-color: white;
  }
</style>

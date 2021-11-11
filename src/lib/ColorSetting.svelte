<script>
  import { styles } from '$lib/styles';

  export let key;
  export let id = '';
</script>

<input {id} type="color" bind:value={$styles[key]} />

<style>
  input {
    border: 2px dotted grey;
    height: calc(100% - 2px);
    width: calc(100% - 2px);
    padding: 0;
    margin: 0;
  }
</style>

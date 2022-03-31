<script>
  import { styles } from '$lib/styles';

  export let key;
  export let id = '';
</script>

<input {id} type="color" bind:value={$styles[key]} />

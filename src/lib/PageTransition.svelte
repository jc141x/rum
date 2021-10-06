<script>
  import { fly } from 'svelte/transition';
  export let refresh = '';
</script>

{#key refresh}
  <main in:fly={{ x: -5, duration: 300, delay: 300 }} out:fly={{ x: 5, duration: 300 }}>
    <slot />
  </main>
{/key}

<style>
  main {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: auto;
  }
</style>
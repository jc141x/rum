<script>
  export let refresh = '';
</script>

{#key refresh}
  <main>
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

<script>
  export let refresh = '';
</script>

{#key refresh}
  <main>
    <slot />
  </main>
{/key}

<script>
  import { cssStyles } from '$lib/styles';

  $: document.documentElement.style.cssText = $cssStyles;
</script>

<div id="top">
  <slot />
</div>

<style lang="postcss">
  #top {
    background-image: linear-gradient(to bottom, var(--bg-top), var(--bg-bottom));
    width: 100vw;
    height: 100vh;
    position: absolute;
    top: 0;
    left: 0;
  }
</style>

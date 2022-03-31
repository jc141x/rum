<script>
  import { cssStyles } from '$lib/styles';

  $: document.documentElement.style.cssText = $cssStyles;
</script>

<div id="top">
  <slot />
</div>


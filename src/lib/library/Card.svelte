<script>
  //import Card from '$lib/Card.svelte';
  import Panel from '$lib/library/Panel.svelte';
  export let game;
  export let selected;
</script>

<Panel {game} {selected}/>
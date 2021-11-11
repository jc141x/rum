<script>
  import { mdiMagnify } from '@mdi/js';
  import Icon from 'mdi-svelte';

  export let query;
</script>

<div class="search-form">
  <span><Icon path={mdiMagnify} /></span>
  <input type="search" bind:value={query} />
</div>

<style>
  .search-form {
    justify-content: flex-end;
    display: inline-flex;
    flex-direction: row;
    align-items: center;

    overflow: hidden;
    position: relative;
    max-width: 2rem;
  }

  [type='search'] {
    position: absolute;
    width: 100%;
    opacity: 0;
    cursor: pointer;
    z-index: 1;
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-size: 90%;
    height: 1.5rem;
  }

  .search-form:focus-within {
    max-width: 100%;
  }

  .search-form:focus-within [type='search'] {
    opacity: 1;
    right: 0;
    position: relative;
  }
  .search-form span {
    position: relative;
    left: 0;
  }
</style>

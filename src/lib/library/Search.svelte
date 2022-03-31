<script>
  import IconMagnify from '~icons/mdi/magnify';

  export let query;
</script>

<div class="search-form">
  <span><IconMagnify /></span>
  <input id="search" type="search" bind:value={query} />
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

  .search-form>input[type='search']#search {
    all: unset;
    position: absolute;
    width: 100%;
    opacity: 0;
    cursor: pointer;
    z-index: 1;
    margin: 0;
    padding: 0;
    box-sizing: border-box;
    font-size: 90%;
    line-height: 1.3rem;
    text-align: left;
    border: 1px solid var(--color-primary);
  }

  .search-form:focus-within {
    max-width: 100%;
  }

  .search-form:focus-within>input[type='search']#search {
    opacity: 1;
    right: 0;
    position: relative;
  }
  .search-form span {
    position: relative;
    left: 0;
  }
</style>
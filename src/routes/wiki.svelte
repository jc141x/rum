<script>
  import { styles } from '$lib/styles';
  import { Pulse } from 'svelte-loading-spinners';
  import { fly } from 'svelte/transition';
  import { afterUpdate, onMount } from 'svelte';
  import command from '$lib/command';
  import showdown from 'showdown';
  import { open } from '../../node_modules/@tauri-apps/api/shell';
  showdown.setFlavor('github');
  const converter = new showdown.Converter();

  let mainDiv;
  let indexDiv;

  let loading = true;
  let mainPage = null;

  const loadPage = async (page) => {
    loading = true;
    let text = '';
    try {
      if (page != null)
        text = await command.misc('get_wiki_page', { page });
    } catch (error) {
      text = error.message;
    }
    loading = false;
    console.log('done');
    return converter.makeHtml(text);
  };

  const updateHtml = (target) => {
    console.log(target);
    target.querySelectorAll('a').forEach((el) => {
      el.onclick = function (event) {
        event.preventDefault();
        if (target.classList.contains('main') && event.target.href.startsWith('http')) {
            open(event.target.href);
        } else {
          // Imagine using "this" in 2021
          const href = this.getAttribute('href');
          window.location.href = '#' + href;
          mainPage = href;
        }
      };
    });
  };

  afterUpdate(() => {
    if (mainDiv) {
      updateHtml(mainDiv);
    }

    if (indexDiv) {
      updateHtml(indexDiv);
    }
  });

  onMount(async () => {
    let anchor = window?.location?.hash?.substr(1);

    if (anchor == null || anchor == '') {
      anchor = 'about.md';
    }

    mainPage = anchor;
  });
</script>

<svelte:head>
  <title>Chad Launcher - Documentation</title>
</svelte:head>

<div class="grid">
  <div>
    {#await loadPage('index.md')}
      <div class="center">
        <Pulse size="60" color={$styles.primary} unit="px" duration="1s" />
      </div>
    {:then html}
      <div class="side" bind:this={indexDiv} transition:fly={{ x: -100, duration: 300 }}>
        {@html html}
      </div>
    {/await}
  </div>
  <div>
    {#await loadPage(mainPage)}
      <div class="center">
        <Pulse size="60" color={$styles.primary} unit="px" duration="1s" />
      </div>
    {:then html}
      <div class="main" bind:this={mainDiv} transition:fly={{ x: 100, duration: 300 }}>
        {@html html}
      </div>
    {/await}
  </div>
</div>

<style>
  .main {
    padding: 10px;
    margin-right: 10px;
    overflow: auto;
    height: 100%;
    width: 100%;
  }

  .grid {
    display: grid;
    grid-template-columns: minmax(300px, max-content) auto;
    grid-gap: 10px;
  }

  .grid > div {
    height: calc(100vh - 100px);
  }

  .center {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }
</style>

<script context="module">
  import { browser, dev } from '$app/env';

  // we don't need any JS on this page, though we'll load
  // it in dev so that we get hot module replacement...
  export const hydrate = dev;

  // ...but if the client-side router is already loaded
  // (i.e. we came here from elsewhere in the app), use it
  export const router = browser;

  // since there's no dynamic data here, we can prerender
  // it so that it gets served as a static asset in prod
  export const prerender = true;
</script>

<script>
  import { onMount } from 'svelte';
  import command from '$lib/command';
  import showdown from 'showdown';
  showdown.setFlavor('github');
  const converter = new showdown.Converter();
  onMount(async () => {
    const text = await command.misc('get_reqs_markdown');
    const html = converter.makeHtml(text);
    const div = document.querySelector('#md');
    div.innerHTML = html;
  });
</script>

<svelte:head>
  <title>Chad Launcher - Documentation</title>
</svelte:head>

<div id="md" />

<style>
  #md {
    padding: 10px;
    margin-right: 10px;
    overflow-y: scroll;
    height: calc(100vh - 100px);
    width: calc(100vw - 10px);
  }
</style>

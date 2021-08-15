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

<svelte:head>
  <title>About</title>
</svelte:head>

<div class="hero has-background-primary block">
  <div class="hero-body">
    <h2 class="has-text-white title is-size-1">GNU/Linux Pirates</h2>
    <p class="has-text-white subitle is-size-3">
      We are a P2P release/repacking group on 1337x. Our objective is to provide an ethical and
      minimalist alternative for the community.
    </p>
  </div>
</div>
<div class="section block content">
  <h2 class="">Rules of the matrix community</h2>
  <ul>
    <li>Respect each other and be civilized.</li>
    <li>No doxxing.</li>
    <li>Links to malware and spyware will be deleted.</li>
    <li>
      Sharing links to other pirated content is only allowed if it's from 1337x.to. (for safety
      purposes)
    </li>
  </ul>
</div>

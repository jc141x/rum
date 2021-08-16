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

<div class="content">
  <h3>GNU/Linux Pirates</h3>
  <h5>
    We are a P2P release/repacking group on 1337x. Our objective is to provide an ethical and
    minimalist alternative for the community.
  </h5>
  <hr />
  <h3>Rules of the matrix community</h3>
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

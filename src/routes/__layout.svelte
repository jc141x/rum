<script>
  import Header from '$lib/Header.svelte';
  import Sidebar from '$lib/Sidebar.svelte';
  import '../styles/halfmoon-variables.min.css';
</script>

<div class="main page-wrapper with-navbar with-sidebar">
  <Header />
  <Sidebar />
  <div class="content-wrapper">
    <slot />
  </div>
</div>

<style>
  :global(.content-wrapper)::-webkit-scrollbar {
    width: 0.5rem;
  }

  :global(.content-wrapper)::-webkit-scrollbar-track {
    background-color: darkgray;
    border-radius: 100px;
  }

  :global(.content-wrapper)::-webkit-scrollbar-thumb {
    background-color: darkslategray;
    border-radius: 100px;
  }
  :global(.sidebar)::-webkit-scrollbar {
    width: 0.5rem;
  }

  :global(.sidebar)::-webkit-scrollbar-track {
    background-color: darkgray;
    border-radius: 100px;
  }

  :global(.sidebar)::-webkit-scrollbar-thumb {
    background-color: darkslategray;
    border-radius: 100px;
  }
</style>

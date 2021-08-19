<script>
  import Header from '$lib/Header.svelte';
  import Sidebar from '$lib/Sidebar.svelte';
  import '../styles/halfmoon-variables.min.css';
  import '../fa/css/all.min.css';
</script>

<div class="main page-wrapper with-navbar">
  <Header />
  <!-- <Sidebar /> -->
  <div class="content-wrapper">
    <slot />
  </div>
</div>

<style>
  :global(.content-wrapper)::-webkit-scrollbar {
    width: 1rem;
  }

  :global(.content-wrapper)::-webkit-scrollbar-track {
    background-color: #25282C;
    border-radius: 0px;
  }

  :global(.content-wrapper)::-webkit-scrollbar-thumb {
    background-color: #191C20;
    border-radius: 0px;
  }
  :global(.sidebar)::-webkit-scrollbar {
    width: 1rem;
  }

  :global(.sidebar)::-webkit-scrollbar-track {
    background-color: #25282C;
    border-radius: 0px;
  }

  :global(.sidebar)::-webkit-scrollbar-thumb {
    background-color: #191C20;
    border-radius: 0px;
  }
</style>

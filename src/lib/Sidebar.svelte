<script>
  import { genres, selectedGenre, mode } from '$lib/store.js';
  const url = 'https://bkftwbhopivmrgzcagus.supabase.co/rest/v1/genre?select=id,name';
  import axios from 'axios';
  axios
    .get(url, {
      headers: {
        apikey:
          'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyNzY0NDc0OCwiZXhwIjoxOTQzMjIwNzQ4fQ.MheXAiuWYFGDuFhfzAnANMzJU2UU4HN2dxwMxGdQd5A'
      }
    })
    .then((response) => ($genres = response.data))
    .catch((error) => console.log(error));
</script>

<aside class="column is-one-fifth-desktop is-one-third-tablet is-full-mobile menu fixed">
  <p class="menu-label">Mode</p>
  <ul class="menu-list">
    <li>
      <a
        on:click={() => {
          $mode = 'list';
        }}
      >
        <span class="icon-text">
          <span class="icon material-icons">view_list</span>
          <span> List </span>
        </span>
      </a>
    </li>
    <li>
      <a
        on:click={() => {
          $mode = 'grid';
        }}
      >
        <span class="icon-text">
          <span class="icon material-icons">grid_view</span>
          <span> Grid </span>
        </span>
      </a>
    </li>
  </ul>
  <p class="menu-label">Genres</p>
  <ul class="menu-list">
    {#each $genres as genre (genre.id)}
      <li>
        <a
          on:click={() => {
            $selectedGenre = genre;
          }}
        >
          <span class="icon-text">
            <span class="icon material-icons">label</span>
            <span>{genre.name} </span>
          </span>
        </a>
      </li>
    {/each}
  </ul>
</aside>

<style>
  .fixed {
    height: 100%;
    overflow: auto;
  }
</style>

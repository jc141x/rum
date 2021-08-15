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
  const url = 'https://bkftwbhopivmrgzcagus.supabase.co/rest/v1/game?select=id,name';
  import axios from 'axios';
  let games = [];
  axios
    .get(url, {
      headers: {
        apikey:
          'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJyb2xlIjoiYW5vbiIsImlhdCI6MTYyNzY0NDc0OCwiZXhwIjoxOTQzMjIwNzQ4fQ.MheXAiuWYFGDuFhfzAnANMzJU2UU4HN2dxwMxGdQd5A'
      }
    })
    .then((response) => response.data)
    .then((data) => (games = data))
    .catch((error) => console.log(error));
</script>

<svelte:head>
  <title>Docs</title>
</svelte:head>

<section class="section">
  <div class="content block">
    <h2>Notes</h2>
    <ul>
      <li>Some of the games are x86 only, which means ARM processors won't be able to run it.</li>
      <li>Our distribution of choice for newbies is EndeavourOS.</li>
      <li>We do not support Non-GNU distributions such as ChromeOS or SteamOS/GamerOS.</li>
    </ul>
    <h2>Requirements</h2>
    <ul>
      <li>Use either EXT4 or BTRFS filesystem.</li>
      <li>Use the terminal to run the scripts.</li>
      <li>Have glibc at 2.31 or higher.</li>
      <li>Have the packages mentioned below.</li>
    </ul>
  </div>
</section>
<section class="section">
  <div class="card">
    <header class="card-header">
      <p class="card-header-title is-size-2">Arch/EndeavourOS/Manjaro</p>
    </header>
    <div class="card-content">
      <div class="content">
        <h2>AMD</h2>
        <pre>
          <code>
# sed -i "/\[multilib\]/,/Include/"'s/^#//' /etc/pacman.conf
# pacman -Syyu

# pacman -S --needed lib32-sdl_ttf lib32-sdl_mixer lib32-sdl_image cabextract dosbox giflib gnutls gst-plugins-base-libs lib32-giflib lib32-gnutls lib32-gst-plugins-base-libs lib32-libcurl-gnutls lib32-libgcrypt lib32-libjpeg-turbo lib32-libpng lib32-libva lib32-libxslt lib32-mpg123 lib32-openal lib32-opencl-icd-loader lib32-sdl2 lib32-vkd3d lib32-vulkan-icd-loader lib32-vulkan-radeon libcurl-gnutls libgcrypt libjpeg-turbo libpng libva libxslt mono mpg123 openal opencl-icd-loader vkd3d vulkan-icd-loader vulkan-radeon wine-staging winetricks zstd

$ git clone https://aur.archlinux.org/paru-bin.git && cd paru-bin && makepkg -si && cd ..
$ paru -S dxvk-bin gamescope
          </code>
        </pre>
        <div class="content">
          <h2>Intel</h2>
          <pre>
          <code>
# sed -i "/\[multilib\]/,/Include/"'s/^#//' /etc/pacman.conf
# pacman -Syyu

# pacman -S --needed lib32-sdl_ttf lib32-sdl_mixer lib32-sdl_image cabextract dosbox giflib gnutls gst-plugins-base-libs lib32-giflib lib32-gnutls lib32-gst-plugins-base-libs lib32-libcurl-gnutls lib32-libgcrypt lib32-libjpeg-turbo lib32-libpng lib32-libva lib32-libxslt lib32-mpg123 lib32-openal lib32-opencl-icd-loader lib32-sdl2 lib32-vkd3d lib32-vulkan-icd-loader lib32-vulkan-intel libcurl-gnutls libgcrypt libjpeg-turbo libpng libva libxslt mono mpg123 openal opencl-icd-loader vkd3d vulkan-icd-loader vulkan-intel wine-staging winetricks zstd

$ git clone https://aur.archlinux.org/paru-bin.git && cd paru-bin && makepkg -si && cd ..
$ paru -S dxvk-bin gamescope
          </code>
        </pre>
        </div>

        <div class="content">
          <h2>NVIDIA (proprietary packages included)</h2>
          <pre>
          <code>
# sed -i "/\[multilib\]/,/Include/"'s/^#//' /etc/pacman.conf
# pacman -Syyu

# pacman -S --needed cabextract dosbox giflib gnutls gst-plugins-base-libs lib32-giflib lib32-gnutls lib32-gst-plugins-base-libs lib32-libcurl-gnutls lib32-libgcrypt lib32-libjpeg-turbo lib32-libpng lib32-libva lib32-libxslt lib32-mpg123 lib32-nvidia-utils lib32-openal lib32-opencl-icd-loader lib32-sdl2 lib32-vkd3d lib32-vulkan-icd-loader libcurl-gnutls libgcrypt libjpeg-turbo libpng libva libxslt mono mpg123 nvidia nvidia-dkms openal opencl-icd-loader vkd3d vulkan-icd-loader wine-staging winetricks zstd

$ git clone https://aur.archlinux.org/paru-bin.git && cd paru-bin && makepkg -si && cd ..
$ paru -S dxvk-bin
          </code>
        </pre>
        </div>
      </div>
    </div>
  </div>
</section>

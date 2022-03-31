import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';
import Icons from 'unplugin-icons/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: [
    preprocess({
      scss: true,
      postcss: true
    })
  ],

  kit: {
    adapter: adapter({
      pages: 'build',
      assets: 'build',
      fallback: 'index.html'
    }),
    vite: {
      plugins: [
        Icons({
          compiler: 'svelte',
          scale: 1,
        })
      ]
    }
    // hydrate the <div id="svelte"> element in src/app.html
  }
};

export default config;

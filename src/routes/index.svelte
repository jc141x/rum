<script>
import { goto } from '$app/navigation';
import { config } from '$lib/store';
  config.get().then(conf => {
      conf.library_paths.length ? goto('/library') : goto('/settings');
  });
</script>

<script>
import { goto } from "$app/navigation";

  const handleKeyPress = async (event) => {
    if (event.ctrlKey && event.key === "Home") {
      goto("/");
    }
    if (event.ctrlKey && event.key === ",") {
      goto("/settings");
    }
  }

  document.addEventListener('keydown', (e) => handleKeyPress(e));
</script>
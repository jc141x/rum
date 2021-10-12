<script>
import { goto } from "$app/navigation";

  const handleKeyPress = async (event) => {
    if (event.ctrlKey && event.key === "Home") {
      goto("/");
    }
    if (event.ctrlKey && event.key === ",") {
      goto("/settings");
    }
    if (event.key === "F1" ) {
      goto("/wiki")
    } 
  }

  document.addEventListener('keydown', (e) => handleKeyPress(e));
</script>
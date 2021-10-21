<script>
import { goto } from "$app/navigation";
import { selectedLocalGame } from "$lib/store";

  const handleKeyPress = async (event) => {
    if (event.ctrlKey && event.key === "Home") {
      goto("/");
    }
    else if (event.ctrlKey && event.key === ",") {
      goto("/settings");
    }
    else if (event.key === "F1" ) {
      goto("/wiki")
    } 
    else if (event.ctrlKey && (event.key === "f" || event.key === "F") && window.location.pathname === "/library") {
      document.querySelector('.search input').focus();
    }
    else if (event.key === "Escape") {
      if (document.activeElement.tagName === "INPUT") {
        document.activeElement.blur();
      }
      $selectedLocalGame =  null;
    }
  }

  document.addEventListener('keydown', (e) => handleKeyPress(e));
</script>
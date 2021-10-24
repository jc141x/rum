<script>
  import { goto } from "$app/navigation";
  import { selectedLocalGame } from "$lib/store";
  var selectedAction = null;
  var buttons = [];

  const handleKeyPress = async (event) => {
    if (event.key === "Enter" && document.activeElement) {
      document.activeElement.click();
    }
    else if (event.ctrlKey && event.key === "Home") {
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
      selectedAction = null;
    }
    else if (window.location.pathname === "/library" && $selectedLocalGame === null) {
      if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
        event.preventDefault();
        $selectedLocalGame = 0;
      }
    }
    else if (window.location.pathname === "/library" && $selectedLocalGame !== null) {
      if (event.key === "ArrowLeft" && $selectedLocalGame > 0) {
        $selectedLocalGame--
        selectedAction = null;
      }
      else if (event.key === "ArrowRight" && $selectedLocalGame < document.querySelectorAll(".card").length -1) {
        $selectedLocalGame++
        selectedAction = null;
      }
      else if (event.shiftKey && event.code === "Tab") {
        event.preventDefault();
        if (document.querySelector(".modal")) {
          buttons = document.querySelectorAll(".modal button");
        }
        else {
          buttons = document.querySelectorAll(".actions button");
        }  
        if (selectedAction === null || selectedAction === 0) {
          selectedAction = buttons.length - 1;
        }
        else {
          selectedAction--;
        }
        buttons[selectedAction].focus();
      }
      else if (event.key === "Tab") {
        event.preventDefault();
        if (document.querySelector(".modal")) {
          buttons = document.querySelectorAll(".modal button");
        }
        else {
          buttons = document.querySelectorAll(".actions button");
        }  
        if  (selectedAction === null || selectedAction === buttons.length - 1) {
          selectedAction = 0;
        }
        else {
          selectedAction++;
        }
        buttons[selectedAction].focus();
      }
    }
  }

  document.addEventListener('keydown', (e) => handleKeyPress(e));
</script>
<script>
  import { goto } from '$app/navigation';
  import { selectedLocalGame } from '$lib/store';
  import { getCurrent } from '../../node_modules/@tauri-apps/api/window';

  var selectedAction = null;
  var buttons = [];

  /**
   * map kayboard input and gamepad input to actions
   * @param {string} key
   * @param {bool} ctrl
   * @param {bool} shift
   * @param {bool} alt
   */
  const handleInput = (key, ctrl = false, shift = false, alt = false) => {
    if (key === 'Enter' || key == 'South') {
      handleInputAction('confirm');
    } else if ((key === 'Home' && ctrl) || key == 'Mode') {
      handleInputAction('home');
    } else if ((key === ',' && ctrl) || key === 'Start') {
      handleInputAction('options');
    } else if (key === 'F1' || key === 'Select') {
      handleInputAction('help');
    } else if ((key.toLowerCase() === 'f' && ctrl) || key === 'North') {
      handleInputAction('search');
    } else if (key === 'Escape' || key === 'East') {
      handleInputAction('back');
    } else if (key === 'ArrowRight' || key === 'DPadRight') {
      handleInputAction('right');
    } else if (key === 'ArrowLeft' || key === 'DPadLeft') {
      handleInputAction('left');
    } else if ((key === 'Tab' && !shift) || key === 'RightTrigger') {
      handleInputAction('next');
    } else if ((key === 'Tab' && shift) || key === 'LeftTrigger') {
      handleInputAction('previous');
    }
  };

  /**
   * handle input actions
   * @param {string} action
   */
  const handleInputAction = async (action) => {
    if (action === 'confirm' && document.activeElement) {
      document.activeElement.click();
    } else if (action === 'home') {
      goto('/');
    } else if (action === 'options') {
      goto('/settings');
    } else if (action === 'help') {
      goto('/wiki');
    } else if (action === 'search' && window.location.pathname === '/library') {
      document.querySelector('.search input').focus();
    } else if (action === 'back') {
      if (document.activeElement.tagName === 'INPUT') {
        document.activeElement.blur();
      }
      $selectedLocalGame = null;
      selectedAction = null;
    } else if (window.location.pathname === '/library' && $selectedLocalGame === null) {
      if (action === 'right' || action === 'left') {
        $selectedLocalGame = 0;
      }
    } else if (window.location.pathname === '/library' && $selectedLocalGame !== null) {
      if (action === 'left' && $selectedLocalGame > 0) {
        $selectedLocalGame--;
        selectedAction = null;
      } else if (
        action === 'right' &&
        $selectedLocalGame < document.querySelectorAll('.card').length - 1
      ) {
        $selectedLocalGame++;
        selectedAction = null;
      } else if (action === 'previous') {
        if (document.querySelector('.modal')) {
          buttons = document.querySelectorAll('.modal button');
        } else {
          buttons = document.querySelectorAll('.actions button');
        }
        if (selectedAction === null || selectedAction === 0) {
          selectedAction = buttons.length - 1;
        } else {
          selectedAction--;
        }
        buttons[selectedAction].focus();
      } else if (action === 'next') {
        if (document.querySelector('.modal')) {
          buttons = document.querySelectorAll('.modal button');
        } else {
          buttons = document.querySelectorAll('.actions button');
        }
        if (selectedAction === null || selectedAction === buttons.length - 1) {
          selectedAction = 0;
        } else {
          selectedAction++;
        }
        buttons[selectedAction].focus();
      }
    }
  };

  document.addEventListener('keydown', (e) => {
    if (e.key === 'Tab' || e.code === 'Tab') {
      e.preventDefault();
    }
    handleInput(e.key == 'Unidentified' ? e.code : e.key, e.ctrlKey, e.shiftKey, e.altKey);
  });
  getCurrent().listen('gamepad', ({ _, payload }) => {
    handleInput(payload.ButtonPressed[0]);
  });
</script>

<script lang="ts">
  import '../lib/theme';
  import HelpIcon from '../lib/HelpIcon.svelte';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  if (typeof window !== 'undefined' && 'serviceWorker' in navigator) {
    window.addEventListener('load', () => {
      navigator.serviceWorker.register('/service-worker.js')
        .then((registration) => {
          console.log('Service Worker registered with scope:', registration.scope);
        })
        .catch((error) => {
          console.error('Service Worker registration failed:', error);
        });
    });
  }

  // Listen for notification clicks/actions
  if (typeof window !== 'undefined') {
    import('@tauri-apps/plugin-notification').then(({ onAction }) => {
      import('@tauri-apps/api/webviewWindow').then(({ WebviewWindow }) => {
        onAction(async (notification) => {
          console.log('Notification action received:', notification);
          const mainWindow = WebviewWindow.getByLabel('main') || WebviewWindow.getCurrent();
          await mainWindow.show();
          await mainWindow.setFocus();
          await mainWindow.unminimize();
        });
      });
    });
  }

  onMount(() => {
    const path = window.location.pathname;
    if (!localStorage.getItem('qw-onboarding-complete') && path !== '/onboarding') {
      goto('/onboarding');
      return;
    }
    if (path === '/') {
      goto('/LogView');
    }
  });
  
  // Determine if help button should show
  $: showHelpButton = typeof window !== 'undefined' && 
    window.location.pathname !== '/onboarding' && 
    window.location.pathname !== '/LogView' && 
    window.location.pathname !== '/InputWindow';
</script>

<a class="skip-link" href="#main-content">Skip to main content</a>

<!-- Header that stays at the top without overlapping content -->
{#if showHelpButton}
  <header class="app-header">
    <div class="header-spacer"></div>
    <button 
      aria-label="Help" 
      title="Help / Onboarding" 
      class="help-button"
      on:click={() => goto('/onboarding')}
    >
      <HelpIcon />
    </button>
  </header>
{/if}

<div class="main-container" id="main-content">
  <slot />
</div>

<style>
  .skip-link {
    position: absolute;
    left: -999px;
    top: 0;
    padding: 0.6rem 1rem;
    background: #fff;
    color: #000;
    border: 2px solid #000;
    z-index: 2000;
  }
  .skip-link:focus {
    left: 1rem;
    top: 1rem;
  }

  .app-header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    padding: 18px;
    background: transparent;
    position: relative;
    z-index: 900;
  }

  .header-spacer {
    flex: 1;
  }

  .help-button {
    background: white;
    border: none;
    cursor: pointer;
    padding: 0;
    margin: 0;
    width: 36px;
    height: 36px;
    border-radius: 50%;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.12);
    transition: background 0.2s, box-shadow 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .help-button:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.18);
  }

  .main-container {
    position: relative;
    min-height: calc(100vh - 72px); /* Subtract header height */
  }

  :global(body) {
    margin: 0;
    font-family: 'Segoe UI', 'Inter', system-ui, -apple-system, sans-serif;
    background: radial-gradient(circle at 20% 20%, #f8f4ff, #f5f9ff 35%, #f7fbf7 65%, #fdfdfd);
    color: #13151a;
    min-height: 100vh;
  }

  :global(button), :global(input), :global(select), :global(textarea) {
    border-radius: 12px;
    border: 1px solid #dfe3eb;
    box-shadow: 0 6px 18px rgba(17, 24, 39, 0.05);
    transition: transform 120ms ease, box-shadow 160ms ease, border-color 160ms ease;
    background: #fff;
  }

  :global(button:hover), :global(input:focus), :global(select:focus), :global(textarea:focus) {
    border-color: #9fb7ff;
    box-shadow: 0 10px 26px rgba(79, 114, 219, 0.16);
    outline: none;
  }

  :global(button:active) {
    transform: translateY(1px);
  }

  :global(textarea), :global(input), :global(select) {
    padding: 10px 12px;
  }

  :global(button) {
    padding: 10px 14px;
    font-weight: 600;
    cursor: pointer;
  }

  :global(.card) {
    background: #fff;
    border-radius: 16px;
    box-shadow: 0 18px 40px rgba(15, 23, 42, 0.08);
    border: 1px solid #e5eaf5;
  }
</style>
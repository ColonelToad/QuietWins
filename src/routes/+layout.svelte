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
          // Get or create the main window
          const mainWindow = WebviewWindow.getByLabel('main') || WebviewWindow.getCurrent();
          // Show and focus the window
          await mainWindow.show();
          await mainWindow.setFocus();
          await mainWindow.unminimize();
          // Optionally navigate to a specific page
          // window.location.href = '/LogView';
        });
      });
    });
  }

  onMount(() => {
    if (!localStorage.getItem('qw-onboarding-complete') && window.location.pathname !== '/onboarding') {
      goto('/onboarding');
    }
  });
</script>
<div style="position:relative; min-height:100vh;">
  {#if typeof window !== 'undefined' && window.location.pathname !== '/onboarding' && window.location.pathname !== '/LogView'}
    <button aria-label="Help" title="Help / Onboarding" style="position:fixed; top:18px; right:18px; z-index:1000; background:none; border:none; cursor:pointer; padding:0; margin:0; width:36px; height:36px; border-radius:50%; box-shadow:0 1px 4px #0002; transition:background 0.2s;" on:click={() => goto('/onboarding')}>
      <HelpIcon />
    </button>
  {/if}
  <slot />
</div>
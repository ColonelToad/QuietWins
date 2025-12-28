<script lang="ts">
  import { settings, type Settings } from '../lib/settings';
  import { get } from 'svelte/store';
  let theme: Settings['theme'];
  let icon: Settings['icon'];
  let notifTime: Settings['notifTime'];
  let notifSound: Settings['notifSound'];
  let shortcut: Settings['shortcut'];
  let font: Settings['font'];
  let autoTag: Settings['autoTag'];
  let privacyLock: Settings['privacyLock'];
  let startup: Settings['startup'];

  $: ({ theme, icon, notifTime, notifSound, shortcut, font, autoTag, privacyLock, startup } = $settings);

  function saveSettings() {
    settings.set({ theme, icon, notifTime, notifSound, shortcut, font, autoTag, privacyLock, startup });
    alert('Settings saved!');
  }

  // Test OS-level notification using Tauri's JS API
  async function testNotification() {
    try {
      const { isPermissionGranted, requestPermission, sendNotification } = await import('@tauri-apps/api/notification');
      let permissionGranted = await isPermissionGranted();
      if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
      }
      if (permissionGranted) {
        sendNotification({
          title: 'Quiet Wins',
          body: "Don't forget to log your quiet win today!",
        });
      } else {
        alert('Notification permission not granted.');
      }
    } catch (e) {
      alert('Tauri notification API not available.');
    }
  }
</script>

<main class="settings">
  <h2>Settings</h2>
  <div class="setting-group">
    <label for="theme-select">App Color Theme:</label>
    <select id="theme-select" bind:value={theme}>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
      <option value="warm">Warm</option>
      <option value="custom">Custom</option>
    </select>
  </div>
  <div class="setting-group">
    <label for="icon-select">App Icon Color:</label>
    <select id="icon-select" bind:value={icon}>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
      <option value="warm">Warm</option>
      <option value="custom">Custom</option>
    </select>
    <span class="note">(Requires app restart)</span>
  </div>
  <div class="setting-group">
    <label for="notif-time">Notification Time:</label>
    <input id="notif-time" type="time" bind:value={notifTime} />
  </div>
  <div class="setting-group">
    <label for="notif-sound">Notification Sound:</label>
    <input id="notif-sound" type="checkbox" bind:checked={notifSound} />
  </div>
  <div class="setting-group">
    <label for="shortcut-input">Global Shortcut:</label>
    <input id="shortcut-input" type="text" bind:value={shortcut} />
    <span class="note">(Requires app restart)</span>
  </div>
  <div class="setting-group">
    <label for="font-select">Font:</label>
    <select id="font-select" bind:value={font}>
      <option value="Garamond">Garamond</option>
      <option value="SF Pro">SF Pro</option>
      <option value="Arial">Arial</option>
      <option value="Custom">Custom</option>
    </select>
  </div>
  <div class="setting-group">
    <label for="auto-tag">Auto-Tagging:</label>
    <input id="auto-tag" type="checkbox" bind:checked={autoTag} />
  </div>
  <div class="setting-group">
    <label for="privacy-lock">Privacy Lock:</label>
    <input id="privacy-lock" type="checkbox" bind:checked={privacyLock} />
  </div>
  <div class="setting-group">
    <label for="startup">Start at Login:</label>
    <input id="startup" type="checkbox" bind:checked={startup} />
    <span class="note">(Requires app restart)</span>
  </div>
  <button on:click={saveSettings}>Save Settings</button>
  <button on:click={testNotification} style="margin-left:1em">Test Notification</button>
</main>

<style>
  .settings {
    max-width: 420px;
    margin: 2rem auto;
    padding: 2rem;
    background: var(--background, #fff);
    color: var(--text, #222);
    border-radius: 16px;
    box-shadow: 0 4px 24px rgba(0,0,0,0.08);
    font-family: inherit;
  }
  .setting-group {
    margin-bottom: 1.2rem;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  label {
    font-weight: 500;
    margin-right: 1rem;
  }
  input[type="text"], select, input[type="time"] {
    font-size: 1rem;
    padding: 0.3rem 0.7rem;
    border-radius: 6px;
    border: 1px solid #ccc;
  }
  button {
    margin-top: 1.5rem;
    font-size: 1.1rem;
    padding: 0.7rem 2.2rem;
    border-radius: 8px;
    border: none;
    background: var(--accent, #CC785C);
    color: #fff;
    cursor: pointer;
    transition: background 0.2s;
  }
  button:hover {
    background: color-mix(in srgb, var(--accent, #CC785C) 80%, #000 20%);
  }
  .note {
    font-size: 0.9em;
    color: #a95e45;
    margin-left: 0.5em;
  }
</style>

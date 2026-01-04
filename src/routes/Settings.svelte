<script lang="ts">
  import { settings, type Settings } from '../lib/settings';
  import { get } from 'svelte/store';
  let theme: Settings['theme'];
  let icon: Settings['icon'];
  let notifTime: Settings['notifTime'];
  let notifSound: Settings['notifSound'];
  let notifEnabled: Settings['notifEnabled'];
  let weeklyRecap: Settings['weeklyRecap'];
  let notifFrequency: Settings['notifFrequency'];
  let dailyMessage: Settings['dailyMessage'];
  let weeklyMessage: Settings['weeklyMessage'];
  let shortcut: Settings['shortcut'];
  let font: Settings['font'];
  let autoTag: Settings['autoTag'];
  let privacyLock: Settings['privacyLock'];
  let startup: Settings['startup'];


$: ({ theme, icon, notifTime, notifSound, notifEnabled, weeklyRecap, notifFrequency, dailyMessage, weeklyMessage, shortcut, font, autoTag, privacyLock, startup } = $settings);

  function saveSettings() {
    settings.set({ theme, icon, notifTime, notifSound, notifEnabled, weeklyRecap, notifFrequency, dailyMessage, weeklyMessage, shortcut, font, autoTag, privacyLock, startup });
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

  async function exportSettingsFile() {
    try {
      const [{ save }, { writeTextFile }] = await Promise.all([
        import('@tauri-apps/api/dialog'),
        import('@tauri-apps/api/fs')
      ]);
      const filePath = await save({
        filters: [{ name: 'JSON', extensions: ['json'] }],
        defaultPath: 'quiet-wins-settings.json'
      });
      if (!filePath) return;
      const current = get(settings);
      await writeTextFile(filePath, JSON.stringify(current, null, 2));
      alert('Settings exported.');
    } catch (e) {
      alert('Failed to export settings.');
      console.error(e);
    }
  }

  async function importSettingsFile() {
    try {
      const [{ open }, { readTextFile }] = await Promise.all([
        import('@tauri-apps/api/dialog'),
        import('@tauri-apps/api/fs')
      ]);
      const filePath = await open({
        multiple: false,
        filters: [{ name: 'JSON', extensions: ['json'] }]
      });
      if (!filePath || Array.isArray(filePath)) return;
      const raw = await readTextFile(filePath);
      const parsed = JSON.parse(raw);
      // Basic shape guard: require notifTime and theme
      if (typeof parsed !== 'object' || !parsed) throw new Error('Invalid file');
      const merged = { ...get(settings), ...parsed };
      settings.set(merged);
      alert('Settings imported.');
    } catch (e) {
      alert('Failed to import settings.');
      console.error(e);
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
    <label for="notif-enabled">Enable Notifications:</label>
    <input id="notif-enabled" type="checkbox" bind:checked={notifEnabled} />
  </div>
  <div class="setting-group">
    <label for="notif-frequency">Notification Frequency:</label>
    <select id="notif-frequency" bind:value={notifFrequency}>
      <option value="daily">Daily</option>
      <option value="off">Off</option>
    </select>
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
    <label for="weekly-recap">Weekly Recap Notification:</label>
    <input id="weekly-recap" type="checkbox" bind:checked={weeklyRecap} />
  </div>
  <div class="setting-group">
    <label for="daily-message">Daily Notification Message:</label>
    <textarea id="daily-message" rows="2" bind:value={dailyMessage} placeholder="Don't forget to log your quiet win today!"></textarea>
  </div>
  <div class="setting-group">
    <label for="weekly-message">Weekly Recap Message (optional):</label>
    <textarea id="weekly-message" rows="2" bind:value={weeklyMessage} placeholder="Great week! Here is your recap."></textarea>
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
  <button aria-label="Send test notification" on:click={testNotification} style="margin-left:1em">Test Notification</button>
  <div style="margin-top:1rem; display:flex; gap:0.8rem; flex-wrap:wrap;">
    <button on:click={exportSettingsFile}>Export Settings</button>
    <button on:click={importSettingsFile}>Import Settings</button>
  </div>
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

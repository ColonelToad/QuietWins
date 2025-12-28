
<script lang="ts">
      let showTagInfo = false;
    // Custom theme colors
    let customBg = $settings.customBg || '#fffbe9';
    let customAccent = $settings.customAccent || '#CC785C';
    let customText = $settings.customText || '#222';

    $: if ($settings.theme === 'custom') {
      document.documentElement.style.setProperty('--background', customBg);
      document.documentElement.style.setProperty('--accent', customAccent);
      document.documentElement.style.setProperty('--text', customText);
      settings.update(s => ({ ...s, customBg, customAccent, customText }));
    }

    function resetCustomTheme() {
      customBg = '#fffbe9';
      customAccent = '#CC785C';
      customText = '#222';
      settings.update(s => ({ ...s, customBg, customAccent, customText }));
    }
  import { settings } from '../../lib/settings';
  import { onMount } from 'svelte';
  let showPasswordModal = false;
  let password = '';
  let confirmPassword = '';
  let passwordError = '';

  // Font dropdown logic (moved from markup)
  let showFontList = false;
  const fontOptions = [
    'Garamond',
    'SF Pro',
    'Arial',
    'Times New Roman',
    'Georgia',
    'Verdana',
    'Courier New',
    'Custom'
  ];
// ...existing code...
  function selectFont(font: string) {
    settings.update(s => ({ ...s, font }));
    showFontList = false;
  }

  // Watch privacyLock and show modal if enabled
  $: if ($settings.privacyLock && !showPasswordModal) {
    showPasswordModal = true;
    password = '';
    confirmPassword = '';
    passwordError = '';
  }

  function handlePasswordSubmit() {
    if (password.length < 4) {
      passwordError = 'Password must be at least 4 characters.';
      return;
    }
    if (password !== confirmPassword) {
      passwordError = 'Passwords do not match.';
      return;
    }
    // Save password to localStorage (for demo; use secure storage in production)
    localStorage.setItem('qw-password', password);
    showPasswordModal = false;
    passwordError = '';
  }

  function handlePasswordCancel() {
    showPasswordModal = false;
    // Uncheck privacyLock if cancelled
    settings.update(s => ({ ...s, privacyLock: false }));
  }
</script>

<main class="settings">
  <h2>Settings</h2>
  <div class="setting-group">
    <label for="theme-select">App Color Theme:</label>
    <select id="theme-select" bind:value={$settings.theme}>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
      <option value="warm">Warm</option>
      <option value="custom">Custom</option>
    </select>
  </div>
  {#if $settings.theme === 'custom'}
    <div class="custom-theme-group">
      <label>Background Color:
        <input type="color" bind:value={customBg} />
      </label>
      <label>Accent Color:
        <input type="color" bind:value={customAccent} />
      </label>
      <label>Text Color:
        <input type="color" bind:value={customText} />
      </label>
      <button type="button" class="reset-btn" on:click={resetCustomTheme}>Reset to Default</button>
      <div class="custom-theme-preview" style="background: {customBg}; color: {customText}; border: 2px solid {customAccent};">
        <span style="color: {customAccent}; font-weight: bold;">Accent</span> &mdash; This is a live preview of your custom theme.
      </div>
    </div>
  {/if}
  <div class="setting-group">
    <label for="icon-select">App Icon Color:</label>
    <select id="icon-select" bind:value={$settings.icon}>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
      <option value="warm">Warm</option>
      <option value="custom">Custom</option>
    </select>
    <span class="note">(Requires app restart)</span>
  </div>
  <div class="setting-group">
    <label for="notif-time">Notification Time:</label>
    <input id="notif-time" type="time" bind:value={$settings.notifTime} />
  </div>
  <div class="setting-group">
    <label for="shortcut-input">Global Shortcut:</label>
    <input id="shortcut-input" type="text" bind:value={$settings.shortcut} />
    <span class="note">(Requires app restart)</span>
  </div>
  <div class="setting-group">
    <label for="font-select">Font:</label>
    <div class="custom-font-dropdown" tabindex="0" on:blur={() => showFontList = false}>
      <div class="selected-font" on:click={() => showFontList = !showFontList} style="font-family: {$settings.font}, serif;">
        {$settings.font}
        <span class="dropdown-arrow">▼</span>
      </div>
      {#if showFontList}
        <ul class="font-list">
          {#each fontOptions as font}
            <li style="font-family: {font}, serif;" class:active={font === $settings.font} on:click={() => selectFont(font)}>{font}</li>
          {/each}
        </ul>
      {/if}
    </div>
  </div>
  <div class="setting-group">
    <label for="auto-tag">Auto-Tagging:</label>
    <input id="auto-tag" type="checkbox" bind:checked={$settings.autoTag} />
  </div>
  {#if showTagInfo}
    <div class="modal-backdrop">
      <div class="modal">
        <h3>How Auto-Tagging Works</h3>
        <p>Auto-tagging helps you organize your wins by suggesting tags based on what you write. For example:</p>
        <ul>
          <li>If you say <b>"I finished my essay draft"</b>, the system tags it as <b>writing</b> and <b>work</b>.</li>
          <li>If you say <b>"I went for a walk with my family"</b>, it tags it as <b>family bonding</b> and <b>casual recreation</b>.</li>
          <li>If you say <b>"I helped a friend with homework"</b>, it tags it as <b>friend</b> and <b>school</b>.</li>
        </ul>
        <p>You can always add, remove, or edit tags before saving.</p>
        <hr style="margin: 1.2em 0; border: none; border-top: 1px solid #eee;">
        <h4>How Tags Power the Graph</h4>
        <p>The tag graph visualizes how your wins are connected by their tags. Each tag becomes a node, and if two tags appear together in a win, they're linked. The more you use and customize tags, the more meaningful and insightful your graph becomes!</p>
        <div class="modal-actions">
          <button on:click={() => showTagInfo = false}>Close</button>
        </div>
      </div>
    </div>
  {/if}
  <div class="setting-group">
    <label for="privacy-lock">Privacy Lock:</label>
    <input id="privacy-lock" type="checkbox" bind:checked={$settings.privacyLock} />
  </div>

  {#if showPasswordModal}
    <div class="modal-backdrop">
      <div class="modal">
        <h3>Set Privacy Lock Password</h3>
        <label>Password:</label>
        <input type="password" bind:value={password} autocomplete="new-password" />
        <label>Retype Password:</label>
        <input type="password" bind:value={confirmPassword} autocomplete="new-password" />
        {#if passwordError}
          <div class="error">{passwordError}</div>
        {/if}
        <div class="modal-actions">
          <button on:click={handlePasswordSubmit}>Set Password</button>
          <button on:click={handlePasswordCancel} class="cancel">Cancel</button>
        </div>
      </div>
    </div>
  {/if}
  <div class="setting-group">
    <label for="startup">Start at Login:</label>
    <input id="startup" type="checkbox" bind:checked={$settings.startup} />
    <span class="note">(Requires app restart)</span>
  </div>
</main>

<style>
    .see-more-btn {
      background: #eee;
      color: #333;
      border: 1px solid #bbb;
      border-radius: 6px;
      padding: 0.2rem 0.8rem;
      font-size: 0.95rem;
      margin-left: 0.7rem;
      cursor: pointer;
      transition: background 0.2s;
    }
    .see-more-btn:hover {
      background: #f3e7e2;
    }
  .custom-theme-group {
    display: flex;
    flex-direction: column;
    gap: 0.7rem;
    margin-bottom: 1.2rem;
    background: #f9f7f6;
    border-radius: 10px;
    padding: 1rem 1.2rem;
    border: 1px solid #eee;
  }
  .custom-theme-group label {
    font-weight: 500;
    display: flex;
    align-items: center;
    gap: 0.7rem;
  }
  .custom-theme-group input[type="color"] {
    width: 2.2em;
    height: 2.2em;
    border: none;
    background: none;
    cursor: pointer;
    border-radius: 6px;
    box-shadow: 0 1px 4px rgba(0,0,0,0.04);
  }
  .reset-btn {
    background: #eee;
    color: #333;
    border: 1px solid #bbb;
    border-radius: 6px;
    padding: 0.4rem 1.2rem;
    font-size: 1rem;
    margin-top: 0.5rem;
    align-self: flex-start;
    cursor: pointer;
  }
  .custom-theme-preview {
    margin-top: 0.7rem;
    padding: 1rem;
    border-radius: 8px;
    font-size: 1.1rem;
    display: flex;
    align-items: center;
    gap: 1.2rem;
  }
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
    gap: 1.2rem;
  }
  .setting-group label {
    min-width: 140px;
    font-weight: 500;
    margin-right: 0.7rem;
    flex-shrink: 0;
  }
  .setting-group input[type="text"],
  .setting-group select,
  .setting-group input[type="time"] {
    font-size: 1rem;
    padding: 0.3rem 0.7rem;
    border-radius: 6px;
    border: 1px solid #ccc;
    flex: 1 1 0;
    min-width: 120px;
    max-width: 220px;
  }
  .setting-group input[type="checkbox"] {
    width: 1.2em;
    height: 1.2em;
    margin-left: 0.5em;
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
    background: #CC785C;
    color: #fff;
    cursor: pointer;
    transition: background 0.2s;
  }
  button:hover {
    background: #a95e45;
  }
  .note {
    font-size: 0.9em;
    color: #a95e45;
    margin-left: 0.5em;
  }
  .modal-backdrop {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0,0,0,0.25);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }
  .modal {
    background: #fff;
    border-radius: 12px;
    box-shadow: 0 4px 24px rgba(0,0,0,0.18);
    padding: 2rem 2.5rem;
    min-width: 320px;
    max-width: 90vw;
    display: flex;
    flex-direction: column;
    gap: 0.7rem;
  }
  .modal label {
    font-weight: 500;
    margin-top: 0.5rem;
  }
  .modal input[type="password"] {
    font-size: 1rem;
    padding: 0.3rem 0.7rem;
    border-radius: 6px;
    border: 1px solid #ccc;
  }
  .modal-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
    justify-content: flex-end;
  }
  .modal-actions .cancel {
    background: #eee;
    color: #333;
    border: 1px solid #bbb;
  }
  .custom-font-dropdown {
    position: relative;
    min-width: 160px;
    user-select: none;
    outline: none;
  }
  .selected-font {
    border: 1px solid #ccc;
    border-radius: 6px;
    padding: 0.3rem 0.7rem;
    background: #fff;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 1rem;
  }
  .dropdown-arrow {
    margin-left: 0.7em;
    font-size: 0.9em;
    color: #888;
  }
  .font-list {
    position: absolute;
    left: 0;
    right: 0;
    top: 110%;
    background: #fff;
    border: 1px solid #ccc;
    border-radius: 6px;
    box-shadow: 0 4px 16px rgba(0,0,0,0.08);
    z-index: 10;
    margin: 0;
    padding: 0.2rem 0;
    list-style: none;
    max-height: 220px;
    overflow-y: auto;
  }
  .font-list li {
    padding: 0.4rem 0.7rem;
    cursor: pointer;
    font-size: 1rem;
    transition: background 0.15s;
  }
  .font-list li.active, .font-list li:hover {
    background: #f3e7e2;
  }
</style>
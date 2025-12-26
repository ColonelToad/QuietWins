<script lang="ts">
  import { onMount } from 'svelte';
  import { getWins, addWin, type Win } from '../lib/tauri';
  import { settings } from '../lib/settings';
  import Settings from 'lucide-svelte/icons/settings';
  import { goto } from '$app/navigation';
  let wins: Win[] = [];
  let newText = '';
  let newTags = '';
  let adding = false;
  let errorMsg: string | null = null;
  let showPasswordModal = false;
  let passwordInput = '';
  let passwordError = '';
  let unlocked = false;

  onMount(async () => {
    if ($settings.privacyLock) {
      showPasswordModal = true;
      unlocked = false;
    } else {
      unlocked = true;
      await loadWins();
    }
  });

  async function loadWins() {
    try {
      wins = await getWins();
    } catch (err) {
      errorMsg = `Failed to load wins: ${typeof err === 'object' && err !== null && 'message' in err ? (err as { message?: string }).message ?? String(err) : String(err)}`;
      console.error('LogView getWins error:', err);
    }
  }

  async function handleAddWin() {
    if (!newText.trim()) return;
    adding = true;
    const date = new Date().toISOString().slice(0, 10);
    try {
      await addWin(date, newText, newTags);
      newText = '';
      newTags = '';
      await loadWins();
    } catch (e) {
      alert('Failed to add win: ' + e);
    } finally {
      adding = false;
    }
  }

  function openSettings() {
    goto('/Settings');
  }

  function handlePasswordSubmit() {
    const stored = localStorage.getItem('qw-password');
    if (!stored || passwordInput !== stored) {
      passwordError = 'Incorrect password.';
      return;
    }
    showPasswordModal = false;
    unlocked = true;
    passwordError = '';
    loadWins();
  }
</script>

<main>
  {#if showPasswordModal}
    <div class="modal-backdrop">
      <div class="modal">
        <h3>Enter Privacy Lock Password</h3>
        <input type="password" bind:value={passwordInput} placeholder="Password" autocomplete="current-password" />
        {#if passwordError}
          <div class="error">{passwordError}</div>
        {/if}
        <div class="modal-actions">
          <button on:click={handlePasswordSubmit}>Unlock</button>
        </div>
      </div>
    </div>
  {/if}
  {#if unlocked}
    <div class="header">
      <h1>Quiet Wins Log</h1>
      <div class="header-actions">
        <button class="graph-btn" on:click={() => goto('/GraphView')} title="View Tag Graph">Graph</button>
        <button class="settings-btn" on:click={openSettings} title="Settings">
          <Settings class="settings-icon" />
        </button>
      </div>
    </div>
    {#if errorMsg}
      <div class="error">{errorMsg}</div>
    {/if}
    {#each wins as win (win.id)}
      <section>
        <div>{win.date}</div>
        <div>{win.text}</div>
        <div class="log-tags"><em>{win.tags}</em></div>
      </section>
    {/each}
  {/if}
</main>

<style>
main {
  font-family: 'SF Pro', 'San Francisco', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Arial, sans-serif;
  background: var(--background);
  color: var(--text);
}
.header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1.5rem;
}
.header-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}
.graph-btn {
  background: var(--accent, #CC785C);
  color: #fff;
  border: none;
  border-radius: 6px;
  padding: 0.4rem 1.2rem;
  font-size: 1rem;
  cursor: pointer;
  transition: background 0.2s;
}
.graph-btn:hover {
  background: color-mix(in srgb, var(--accent, #CC785C) 80%, #000 20%);
}
.settings-btn {
  background: none;
  border: none;
  cursor: pointer;
  padding: 0.2rem;
  border-radius: 50%;
  transition: background 0.2s;
}
.settings-btn:hover {
  background: #f3e7e2;
}
.add-win-form {
  display: flex;
  gap: 0.7rem;
  margin-bottom: 1.5rem;
  align-items: center;
}
.add-win-text {
  flex: 2;
  font-size: 1rem;
  padding: 0.3rem 0.7rem;
  border-radius: 6px;
  border: 1px solid #ccc;
}
.add-win-tags {
  flex: 1;
  font-size: 1rem;
  padding: 0.3rem 0.7rem;
  border-radius: 6px;
  border: 1px solid #ccc;
}
.add-win-form button[type="submit"] {
  font-size: 1rem;
  padding: 0.3rem 1.2rem;
  border-radius: 6px;
  border: none;
  background: var(--accent, #CC785C);
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}
.add-win-form button[type="submit"]:hover {
  background: color-mix(in srgb, var(--accent, #CC785C) 80%, #000 20%);
}
section {
  margin-bottom: 1.5rem;
  padding: 1rem;
  border-radius: 8px;
  background: rgba(0,0,0,0.03);
}
.log-tags em {
  color: #a95e45;
  font-style: italic;
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
</style>
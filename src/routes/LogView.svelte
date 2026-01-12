<script lang="ts">
  import { onMount } from 'svelte';
  import { getWins, addWin, getWinsWithChains, updateWin as updateWinApi, deleteWin as deleteWinApi, getDeletedWins, restoreWin, type WinWithChain } from '../lib/tauri';
  import { settings } from '../lib/settings';
  import Settings from 'lucide-svelte/icons/settings';
  let wins: WinWithChain[] = [];
  let deletedWins: any[] = [];
  let newText = '';
  let newTags = '';
  let adding = false;
  let errorMsg: string | null = null;
  let showPasswordModal = false;
  let passwordInput = '';
  let passwordError = '';
  let unlocked = false;
  let showTrash = false;

  // Group wins by chain_id for rendering
  const maxChainId = Number.MAX_SAFE_INTEGER;
  const maxChainIdStr = String(maxChainId);
  // Subtle grouping – no distinct color coding
  const chainColors = ['transparent'];
  function groupByChain(list: WinWithChain[]) {
    const groups: Record<number, WinWithChain[]> = {};
    for (const win of list) {
      if (!groups[win.chain_id]) groups[win.chain_id] = [];
      groups[win.chain_id].push(win);
    }
    return Object.entries(groups)
      .sort(([a], [b]) => {
        if (a === b) return 0;
        if (a === maxChainIdStr) return 1;
        if (b === maxChainIdStr) return -1;
        return Number(a) - Number(b);
      })
      .map(([chain_id, chainWins]) => ({ chain_id: Number(chain_id), chainWins }));
  }
  function chainColor(chain_id: number) {
    return 'transparent';
  }

  // Undo/redo stacks for edits
  type WinSnapshot = { id: number; date: string; text: string; tags: string };
  type EditAction = { before: WinSnapshot | null; after: WinSnapshot | null };
  let undoStack: EditAction[] = [];
  let redoStack: EditAction[] = [];
  let editingId: number | null = null;
  let editText = '';
  let editTags = '';

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
      wins = await getWinsWithChains();
    } catch (err) {
      errorMsg = `Failed to load wins: ${typeof err === 'object' && err !== null && 'message' in err ? (err as { message?: string }).message ?? String(err) : String(err)}`;
      console.error('LogView getWins error:', err);
    }
  }

  async function loadDeletedWins() {
    try {
      deletedWins = await getDeletedWins();
    } catch (err) {
      errorMsg = `Failed to load deleted wins: ${String(err)}`;
      console.error('LogView getDeletedWins error:', err);
    }
  }

  async function toggleTrash() {
    showTrash = !showTrash;
    if (showTrash) {
      await loadDeletedWins();
    }
  }

  async function restoreDeletedWin(win: any) {
    try {
      await restoreWin(win.id);
      await loadDeletedWins();
      await loadWins();
      errorMsg = null;
    } catch (err) {
      errorMsg = `Failed to restore: ${String(err)}`;
    }
  }

  async function handleAddWin() {
    if (!newText.trim()) return;
    adding = true;
    const date = new Date().toISOString().slice(0, 10);
    try {
      await addWin({ text: newText, tags: newTags, date });
      newText = '';
      newTags = '';
      await loadWins();
    } catch (err) {
      errorMsg = `Failed to add win: ${typeof err === 'object' && err !== null && 'message' in err ? (err as { message?: string }).message ?? String(err) : String(err)}`;
      console.error('handleAddWin error:', err);
    } finally {
      adding = false;
    }
  }

  function beginEdit(win: WinWithChain) {
    editingId = win.id;
    editText = win.text;
    editTags = win.tags;
  }

  function cancelEdit() {
    editingId = null;
    editText = '';
    editTags = '';
  }

  async function saveEdit(win: WinWithChain) {
    if (editingId !== win.id) return;
    const before: WinSnapshot = { id: win.id, date: win.date, text: win.text, tags: win.tags };
    const after: WinSnapshot = { id: win.id, date: win.date, text: editText, tags: editTags };
    try {
      await updateWinApi({ id: win.id, date: win.date, text: editText, tags: editTags });
      undoStack.push({ before, after });
      redoStack = [];
      editingId = null;
      await loadWins();
    } catch (err) {
      errorMsg = `Failed to save edit: ${String(err)}`;
    }
  }

  async function undoLast() {
    const action = undoStack.pop();
    if (!action) return;
    try {
      if (action.before && action.after) {
        // edit revert
        await updateWinApi({ id: action.before.id, date: action.before.date, text: action.before.text, tags: action.before.tags });
      } else if (action.before && !action.after) {
        // deletion undo: reinsert by add_win then update id? Not feasible with AUTOINCREMENT; instead, skip for now.
      } else if (!action.before && action.after) {
        // creation undo (not used here)
      }
      redoStack.push(action);
      await loadWins();
    } catch (err) {
      errorMsg = `Failed to undo: ${String(err)}`;
    }
  }

  async function redoLast() {
    const action = redoStack.pop();
    if (!action) return;
    try {
      if (action.before && action.after) {
        await updateWinApi({ id: action.after.id, date: action.after.date, text: action.after.text, tags: action.after.tags });
      }
      undoStack.push(action);
      await loadWins();
    } catch (err) {
      errorMsg = `Failed to redo: ${String(err)}`;
    }
  }

  async function deleteWinEntry(win: WinWithChain) {
    const before: WinSnapshot = { id: win.id, date: win.date, text: win.text, tags: win.tags };
    try {
      await deleteWinApi(win.id);
      undoStack.push({ before, after: null });
      redoStack = [];
      await loadWins();
    } catch (err) {
      errorMsg = `Failed to delete: ${String(err)}`;
    }
  }

  async function handlePasswordSubmit() {
    const stored = $settings.privacyPassword;
    if (!stored || passwordInput !== stored) {
      passwordError = 'Incorrect password.';
      return;
    }
    showPasswordModal = false;
    unlocked = true;
    passwordError = '';
    await loadWins();
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
      <a class="graph-btn" href="/GraphView" title="View Tag Graph">Graph</a>
      <a class="settings-btn icon-btn" href="/Settings" title="Settings" aria-label="Settings">
        <Settings class="settings-icon" />
      </a>
      <!-- Help button removed per request -->
      <button class="undo-btn" on:click={undoLast} disabled={undoStack.length === 0} title="Undo last edit">Undo</button>
      <button class="redo-btn" on:click={redoLast} disabled={redoStack.length === 0} title="Redo last edit">Redo</button>
      <button class="trash-btn" on:click={toggleTrash} title="View deleted wins">🗑️ Trash</button>
    </div>
    </div>
    {#if errorMsg}
      <div class="error">{errorMsg}</div>
    {/if}
    {#if showTrash}
      <div class="trash-section">
        <h2>Trash (48-hour retention)</h2>
        {#if deletedWins.length === 0}
          <p class="empty-trash">No deleted wins. Your trash is clean!</p>
        {:else}
          <div class="deleted-wins-list">
            {#each deletedWins as win (win.id)}
              <div class="deleted-win-card">
                <div class="deleted-win-header">
                  <div class="deleted-win-date">{win.date}</div>
                  <button class="restore-btn" on:click={() => restoreDeletedWin(win)} title="Restore this win">↻ Restore</button>
                </div>
                <div class="deleted-win-text">{win.text}</div>
                <div class="deleted-win-tags"><em>{win.tags}</em></div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {:else}
      {#if errorMsg}
      <div class="error">{errorMsg}</div>
    {/if}
    {#each groupByChain(wins) as { chain_id, chainWins }, i}
      <div class="chain-section" style="background: {chainColor(chain_id)};">
        {#each chainWins as win (win.id)}
          <section>
            <div class="win-row">
              <div class="win-date">{win.date}</div>
              <div class="win-actions">
                {#if editingId === win.id}
                  <button on:click={() => saveEdit(win)}>Save</button>
                  <button class="cancel" on:click={cancelEdit}>Cancel</button>
                {:else}
                  <button on:click={() => beginEdit(win)}>Edit</button>
                  <button class="cancel" on:click={() => deleteWinEntry(win)}>Delete</button>
                {/if}
              </div>
            </div>
            {#if editingId === win.id}
              <textarea class="edit-text" bind:value={editText} rows="3"></textarea>
              <input class="edit-tags" type="text" bind:value={editTags} placeholder="Tags" />
            {:else}
              <div>{win.text}</div>
              <div class="log-tags"><em>{win.tags}</em></div>
            {/if}
          </section>
        {/each}
      </div>
    {/each}
    {/if}
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
  padding: 20px 24px;
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
.icon-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: none;
  border: none;
  border-radius: 50%;
  cursor: pointer;
  transition: background 0.2s;
}
.icon-btn:hover { background: #f3e7e2; }
.settings-icon { width: 22px; height: 22px; }
.undo-btn, .redo-btn {
  background: #eee;
  border: 1px solid #ccc;
  border-radius: 6px;
  padding: 0.35rem 0.8rem;
  cursor: pointer;
}
.undo-btn:disabled, .redo-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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
/* ...existing code... */
.chain-section {
  border-radius: 12px;
  margin-bottom: 1rem;
  padding: 0.25rem 0.25rem 0.25rem 0.25rem;
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
  margin-bottom: 0.8rem;
  padding: 0.8rem;
  border-radius: 8px;
  background: #fff;
  border: 1px solid #eee;
}
.win-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}
.win-date {
  font-weight: 600;
}
.win-actions button {
  margin-left: 0.4rem;
}
.win-actions .cancel {
  background: #f6e3de;
  border: 1px solid #e3c9c0;
}
.edit-text {
  width: 100%;
  margin: 0.5rem 0;
  padding: 0.6rem;
  border-radius: 6px;
  border: 1px solid #ccc;
  font-family: inherit;
}
.edit-tags {
  width: 100%;
  padding: 0.5rem 0.6rem;
  border-radius: 6px;
  border: 1px solid #ccc;
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
  .trash-btn {
    background: #eee;
    border: 1px solid #ccc;
    border-radius: 6px;
    padding: 0.35rem 0.8rem;
    cursor: pointer;
    font-size: 1rem;
  }
  .trash-btn:hover {
    background: #f0e6e0;
  }
  .trash-section {
    background: #f9f3f1;
    border-radius: 12px;
    padding: 1.5rem;
    margin-top: 1.5rem;
    border: 1px solid #e8d4ca;
  }
  .trash-section h2 {
    margin-top: 0;
    color: #a95e45;
    font-size: 1.4rem;
  }
  .empty-trash {
    text-align: center;
    color: #999;
    padding: 2rem 1rem;
    font-style: italic;
  }
  .deleted-wins-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .deleted-win-card {
    background: #fff;
    border: 1px solid #e8d4ca;
    border-radius: 8px;
    padding: 1rem;
  }
  .deleted-win-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  .deleted-win-date {
    font-weight: 600;
    color: #666;
  }
  .restore-btn {
    background: #d4e8d4;
    border: 1px solid #b8d9b8;
    border-radius: 6px;
    padding: 0.3rem 0.8rem;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background 0.2s;
  }
  .restore-btn:hover {
    background: #bfd9bf;
  }
  .deleted-win-text {
    color: #333;
    margin: 0.5rem 0;
  }
  .deleted-win-tags {
    color: #a95e45;
    font-size: 0.9rem;
    margin-top: 0.5rem;
  }
</style>
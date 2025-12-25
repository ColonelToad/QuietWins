<script>
  import { createEventDispatcher } from 'svelte';
  import { addWin } from '../lib/tauri';
  import Settings from 'lucide-svelte/icons/settings';
  import { goto } from '$app/navigation';
  const dispatch = createEventDispatcher();
  let text = '';
  let showBanner = false;

  async function save() {
    const date = new Date().toISOString().slice(0, 10);
    try {
      await addWin(date, text, '');
      dispatch('save', { text });
      text = '';
      showBanner = true;
      setTimeout(() => showBanner = false, 2000);
    } catch (e) {
      alert('Failed to save win: ' + e);
    }
  }
  function cancel() {
    dispatch('cancel');
  }
  function openSettings() {
    goto('/Settings');
  }
</script>

<div class="input-window">
  <button class="settings-btn" on:click={openSettings} title="Settings">
    <Settings class="settings-icon" />
  </button>
  <textarea bind:value={text} placeholder="Log your quiet win..." rows="4"></textarea>
  <div class="actions-row">
    <div class="actions">
      <button on:click={save}>Save</button>
      <button on:click={cancel}>Cancel</button>
    </div>
    {#if showBanner}
      <div class="banner bottom-left">Win logged!</div>
    {/if}
  </div>
</div>

<style>
.input-window {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  background: var(--background);
  color: var(--text);
  border-radius: 12px;
  box-shadow: 0 4px 24px rgba(0,0,0,0.12);
  padding: 2rem;
  min-width: 320px;
  z-index: 9999;
}
 .settings-btn {
   position: absolute;
   top: 1rem;
   left: 1rem;
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
  .actions-row {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-end;
    margin-top: 1rem;
    gap: 1.2rem;
  }
  .banner.bottom-left {
    position: relative;
    left: 0;
    background: #CC785C;
    color: #fff;
    padding: 0.5rem 1.5rem;
    border-radius: 8px;
    font-size: 1.1rem;
    font-family: inherit;
    box-shadow: 0 2px 8px rgba(0,0,0,0.08);
    animation: fadein 0.3s;
    margin-left: 0.5rem;
  }
  @keyframes fadein {
    from { opacity: 0; }
    to { opacity: 1; }
  }
 @keyframes fadein {
   from { opacity: 0; }
   to { opacity: 1; }
 }
textarea {
  width: 100%;
  margin-bottom: 1rem;
  font-size: 1rem;
  font-family: inherit;
  border-radius: 6px;
  border: 1px solid #ccc;
  padding: 0.5rem;
}
.actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
}
button {
  font-size: 1rem;
  padding: 0.5rem 1.2rem;
  border-radius: 6px;
  border: none;
  background: #007aff;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}
button:last-child {
  background: #ccc;
  color: #222;
}
button:hover {
  background: #005bb5;
}
</style>

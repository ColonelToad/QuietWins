<script>
  import { createEventDispatcher } from 'svelte';
  import { addWin } from '../lib/tauri';
  const dispatch = createEventDispatcher();
  let text = '';
  let tags = '';

  async function save() {
    const date = new Date().toISOString().slice(0, 10);
    try {
      await addWin(date, text, tags);
      dispatch('save', { text, tags });
      text = '';
      tags = '';
    } catch (e) {
      alert('Failed to save win: ' + e);
    }
  }
  function cancel() {
    dispatch('cancel');
  }
</script>

<div class="input-window">
  <textarea bind:value={text} placeholder="Log your quiet win..." rows="4"></textarea>
  <input type="text" bind:value={tags} placeholder="Tags (comma separated)" />
  <div class="actions">
    <button on:click={save}>Save</button>
    <button on:click={cancel}>Cancel</button>
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
textarea, input {
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

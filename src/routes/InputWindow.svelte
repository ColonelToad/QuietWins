<script lang="ts">
    import { normalizeTag, uniqueTags, didYouMean } from '../lib/tagUtils';
    import { getUserTagPrefs, addUserTagPref } from '../lib/userTagPrefs';
    let tags = [];
    let tagInput = '';
    let tagError = '';
    let allKnownTags: string[] = [];

    // Suggest tags based on user prefs and text
    function suggestTags(text: string): string[] {
      // For demo, just split on space and check user prefs
      const prefs = getUserTagPrefs();
      let result: string[] = [];
      for (const word of text.split(/\s+/)) {
        const t = prefs[word.toLowerCase()];
        if (t) result.push(...t);
      }
      return uniqueTags(result);
    }

    // When text changes, update tags
    $: if (editorRef && editorRef.innerText) {
      const suggested = suggestTags(editorRef.innerText);
      if (suggested.length && tags.length === 0) tags = suggested;
    }

    function addTag() {
      const norm = normalizeTag(tagInput);
      if (!norm) return;
      if (tags.some(t => normalizeTag(t) === norm)) {
        tagError = '';
        tagInput = '';
        return;
      }
      // Spellcheck: suggest correction if close to known tag
      const suggestion = didYouMean(norm, allKnownTags);
      if (suggestion && suggestion !== norm) {
        tagError = `Did you mean "${suggestion}"?`;
        return;
      }
      tags = uniqueTags([...tags, tagInput]);
      tagInput = '';
      tagError = '';
    }

    function removeTag(idx: number) {
      tags = tags.filter((_, i) => i !== idx);
    }

    // Save user tag prefs if tags were changed
    function saveTagPrefs(text: string, tags: string[]) {
      for (const word of text.split(/\s+/)) {
        addUserTagPref(word, tags.map(normalizeTag));
      }
    }
  import { createEventDispatcher, onMount } from 'svelte';
  import { addWin } from '../lib/tauri';
  import Settings from 'lucide-svelte/icons/settings';
  import { goto } from '$app/navigation';
  const dispatch = createEventDispatcher();
  let text = '';
  let showBanner = false;
  let editorRef;

  onMount(() => {
    if (editorRef && !editorRef.innerHTML.trim()) {
      editorRef.innerHTML = '<div><span class="bullet">•</span><span class="text-content"> </span></div>';
      placeCaretAtEnd();
    }
  });

  async function save() {
    const date = new Date().toISOString().slice(0, 10);
    const textContent = editorRef ? editorRef.innerText : text;
    try {
      await addWin(date, textContent, tags.map(normalizeTag).join(','));
      saveTagPrefs(textContent, tags);
      dispatch('save', { text: textContent });
      if (editorRef) {
        editorRef.innerHTML = '<div><span class="bullet">•</span><span class="text-content"> </span></div>';
        placeCaretAtEnd();
      }
      tags = [];
      tagInput = '';
      tagError = '';
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

  function handleEditorKeydown(e) {
    if (e.key === 'Enter') {
      if (e.shiftKey) {
        e.preventDefault();
        const selection = window.getSelection();
        const range = selection.getRangeAt(0);
        
        const newLine = document.createElement('div');
        newLine.innerHTML = '<span class="bullet">•</span><span class="text-content"> </span>';
        
        range.deleteContents();
        range.insertNode(newLine);
        
        const textSpan = newLine.querySelector('.text-content');
        range.setStart(textSpan, 0);
        range.collapse(true);
        selection.removeAllRanges();
        selection.addRange(range);
      } else {
        e.preventDefault();
        save();
      }
    }
  }

  function placeCaretAtEnd() {
    if (!editorRef) return;
    const range = document.createRange();
    const sel = window.getSelection();
    const lastChild = editorRef.lastChild;
    if (lastChild) {
      const textSpan = lastChild.querySelector('.text-content');
      if (textSpan) {
        range.setStart(textSpan, 0);
        range.collapse(true);
        sel.removeAllRanges();
        sel.addRange(range);
      }
    }
    editorRef.focus();
  }
</script>

<div class="input-window">
  <!-- Removed settings gear icon button -->
  <div class="logwin-input-row">
    <div
      bind:this={editorRef}
      class="logwin-input"
      contenteditable="true"
      on:keydown={handleEditorKeydown}
      role="textbox"
      tabindex="0"
      aria-label="Log your quiet win..."
    ></div>
  </div>
  <div class="tag-editor">
    {#each tags as tag, i}
      <span class="tag-pill">{tag} <button class="remove-tag" on:click={() => removeTag(i)} title="Remove tag">×</button></span>
    {/each}
    <input
      class="tag-input"
      type="text"
      placeholder="Add tag..."
      bind:value={tagInput}
      on:keydown={(e) => { if (e.key === 'Enter') addTag(); }}
    />
    <button class="add-tag-btn" on:click={addTag}>Add</button>
    {#if tagError}
      <span class="tag-error">{tagError}</span>
    {/if}
  </div>
  {#if showBanner}
    <div class="banner">Win logged!</div>
  {/if}
  
  <div class="footer">
    <div class="keybinds">
      <span class="keybind-text"><kbd>Enter</kbd> logs win • <kbd>Shift+Enter</kbd> new entry</span>
    </div>
    <button class="settings-btn-text" on:click={openSettings}>Settings</button>
  </div>
</div>

<style>
  .tag-editor {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
    margin-bottom: 1rem;
  }
  .tag-pill {
    background: #f3e7e2;
    color: #a95e45;
    border-radius: 16px;
    padding: 0.2rem 0.9rem 0.2rem 0.7rem;
    font-size: 0.98rem;
    display: flex;
    align-items: center;
    gap: 0.3rem;
    border: 1px solid #e0cfc7;
  }
  .remove-tag {
    background: none;
    border: none;
    color: #a95e45;
    font-size: 1.1em;
    cursor: pointer;
    margin-left: 0.2em;
  }
  .tag-input {
    min-width: 90px;
    font-size: 1rem;
    border-radius: 8px;
    border: 1px solid #ccc;
    padding: 0.2rem 0.7rem;
  }
  .add-tag-btn {
    font-size: 0.95rem;
    padding: 0.2rem 0.9rem;
    border-radius: 8px;
    border: 1px solid #bbb;
    background: #fff;
    color: #a95e45;
    cursor: pointer;
    margin-left: 0.3rem;
    transition: background 0.2s;
  }
  .add-tag-btn:hover {
    background: #f3e7e2;
  }
  .tag-error {
    color: #a95e45;
    font-size: 0.92em;
    margin-left: 0.7em;
  }
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


.logwin-input-row {
  display: flex;
  align-items: flex-start;
  margin-bottom: 1.2rem;
}

.logwin-input {
  width: 100%;
  min-height: 80px;
  max-height: 300px;
  overflow-y: auto;
  font-size: 1.08rem;
  font-family: inherit;
  border-radius: 10px;
  border: 1px solid #ccc;
  padding: 0.7rem 1rem;
  background: #f9f7f6;
  box-shadow: 0 2px 8px rgba(0,0,0,0.04);
  line-height: 1.5;
}

.logwin-input:focus {
  outline: none;
  border-color: var(--accent, #CC785C);
}

.logwin-input :global(.bullet) {
  color: #CC785C;
  font-weight: bold;
  margin-right: 0.5rem;
}

.logwin-input :global(div) {
  margin: 0.2rem 0;
}

.logwin-input:empty:before {
  content: 'Log your quiet win...';
  color: #999;
  font-style: italic;
}

.banner {
  background: var(--accent, #CC785C);
  color: #fff;
  padding: 0.5rem 1.5rem;
  border-radius: 8px;
  font-size: 1.1rem;
  font-family: inherit;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  animation: fadein 0.3s;
  text-align: center;
  margin-top: 1rem;
}

.footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 1rem;
  gap: 1rem;
}

.keybinds {
  flex: 1;
}

.keybind-text {
  font-style: italic;
  font-size: 0.85rem;
  color: #666;
}

.keybind-text kbd {
  font-family: inherit;
  font-style: normal;
  background: #e8e8e8;
  padding: 0.1rem 0.4rem;
  border-radius: 3px;
  font-size: 0.8rem;
  border: 1px solid #ccc;
}

.settings-btn-text {
  font-size: 0.9rem;
  padding: 0.4rem 1rem;
  border-radius: 6px;
  border: 1px solid #ccc;
  background: #fff;
  color: #333;
  cursor: pointer;
  transition: all 0.2s;
  white-space: nowrap;
}

.settings-btn-text:hover {
  background: color-mix(in srgb, var(--accent, #CC785C) 80%, #fff 20%);
  border-color: var(--accent, #CC785C);
}

@keyframes fadein {
  from { opacity: 0; }
  to { opacity: 1; }
}
</style>
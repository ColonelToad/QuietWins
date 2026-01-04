<script lang="ts">
  import { normalizeTag, uniqueTags, didYouMean } from '../lib/tagUtils';
  import { getUserTagPrefs, addUserTagPref } from '../lib/userTagPrefs';
  import { createEventDispatcher, onMount } from 'svelte';
  import { addWin, suggestTagsForText } from '../lib/tauri';
  import Settings from 'lucide-svelte/icons/settings';
  import HelpCircle from 'lucide-svelte/icons/help-circle';
  import { goto } from '$app/navigation';
  
  const dispatch = createEventDispatcher();
  
  let tagsByLine = {}; // Store tags per line: { lineIndex: ['tag1', 'tag2'] }
  let currentLineIndex = 0;
  let tagInput = '';
  let tagError = '';
  let allKnownTags: string[] = [];
  let showBanner = false;
  let editorRef;
  let modalRef: HTMLDivElement | null = null;

  // Extract all lines/bullets from editor
  function getEditorLines(): { index: number, text: string }[] {
    if (!editorRef) return [];
    const lines = [];
    const divs = editorRef.querySelectorAll('div');
    divs.forEach((div, idx) => {
      const textSpan = div.querySelector('.text-content');
      const text = textSpan ? textSpan.textContent.trim() : '';
      if (text) {
        lines.push({ index: idx, text });
      }
    });
    return lines;
  }

  // Suggest tags based on user prefs and text
  function suggestTags(text: string): string[] {
    const prefs = getUserTagPrefs();
    let result: string[] = [];
    for (const word of text.split(/\s+/)) {
      const t = prefs[word.toLowerCase()];
      if (t) result.push(...t);
    }
    return uniqueTags(result);
  }

  // Update tags when user moves to a new line or when current line text changes
  async function updateCurrentLineTags() {
    const lines = getEditorLines();
    const currentLine = lines[currentLineIndex];

    if (currentLine && currentLine.text) {
      // Use backend for tag suggestion
      const suggested = await suggestTagsForText(currentLine.text);
      // Only auto-populate if this line doesn't have tags yet
      if (suggested.length && (!tagsByLine[currentLineIndex] || tagsByLine[currentLineIndex].length === 0)) {
        tagsByLine[currentLineIndex] = suggested;
      }
    }

    // Update the tag input to show current line's tags
    const currentTags = tagsByLine[currentLineIndex] || [];
    tagInput = currentTags.join(', ');
  }

  // Detect which line the cursor is on
  function detectCurrentLine() {
    if (!editorRef) return;
    
    const selection = window.getSelection();
    if (!selection.rangeCount) return;
    
    const range = selection.getRangeAt(0);
    let node = range.startContainer;
    
    // Walk up to find the div parent
    while (node && node !== editorRef) {
      if (node.parentNode === editorRef && node.nodeType === Node.ELEMENT_NODE) {
        const divs = Array.from(editorRef.querySelectorAll('div'));
        const index = divs.indexOf(node);
        if (index !== -1 && index !== currentLineIndex) {
          currentLineIndex = index;
          updateCurrentLineTags();
        }
        return;
      }
      node = node.parentNode;
    }
  }

  // Handle tag input changes - update the current line's tags
  function handleTagInputChange() {
    const tags = tagInput
      .split(',')
      .map(t => t.trim())
      .filter(t => t);
    
    tagsByLine[currentLineIndex] = tags;
  }

  function addTag() {
    const norm = normalizeTag(tagInput);
    if (!norm) return;
    
    const currentTags = tagsByLine[currentLineIndex] || [];
    if (currentTags.some(t => normalizeTag(t) === norm)) {
      tagError = '';
      return;
    }
    
    // Spellcheck
    const suggestion = didYouMean(norm, allKnownTags);
    if (suggestion && suggestion !== norm) {
      tagError = `Did you mean "${suggestion}"?`;
      return;
    }
    
    tagsByLine[currentLineIndex] = uniqueTags([...currentTags, tagInput]);
    tagInput = tagsByLine[currentLineIndex].join(', ');
    tagError = '';
  }

  function saveTagPrefs(text: string, tags: string[]) {
    for (const word of text.split(/\s+/)) {
      addUserTagPref(word, tags.map(normalizeTag));
    }
  }

  onMount(() => {
    if (editorRef && !editorRef.innerHTML.trim()) {
      editorRef.innerHTML = '<div><span class="bullet">•</span><span class="text-content"> </span></div>';
      placeCaretAtEnd();
    }
  });

  // Popup state for tag review
  let showTagReview = false;
  let reviewLines = [];
  let reviewTagsByLine = {};
  let reviewError = '';

  $: if (showTagReview && modalRef) {
    modalRef.focus();
  }

  function handleModalKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      event.stopPropagation();
      cancelTagReview();
    }
  }

  async function save() {
    const date = new Date().toISOString().slice(0, 10);
    const lines = getEditorLines();
    // Prepare review state
    reviewLines = lines.map(l => ({ ...l }));
    reviewTagsByLine = { ...tagsByLine };
    showTagReview = true;
    reviewError = '';
  }

  async function confirmTagReview() {
    const date = new Date().toISOString().slice(0, 10);
    try {
      for (const line of reviewLines) {
        const lineTags = reviewTagsByLine[line.index] || [];
        const tagsString = lineTags.map(normalizeTag).join(',');
        await addWin(date, line.text, tagsString);
        saveTagPrefs(line.text, lineTags);
      }
      dispatch('save', { count: reviewLines.length });
      // Reset editor
      if (editorRef) {
        editorRef.innerHTML = '<div><span class="bullet">•</span><span class="text-content"> </span></div>';
        placeCaretAtEnd();
      }
      tagsByLine = {};
      currentLineIndex = 0;
      tagInput = '';
      tagError = '';
      showBanner = true;
      setTimeout(() => showBanner = false, 2000);
      showTagReview = false;
    } catch (e) {
      reviewError = 'Failed to save wins: ' + e;
    }
  }

  function cancelTagReview() {
    showTagReview = false;
  }

  function cancel() {
    dispatch('cancel');
  }

  function openSettings() {
    goto('/Settings');
  }

  function openHelp() {
    goto('/onboarding');
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
        
        // Update line index and clear tags for new line
        currentLineIndex++;
        tagInput = '';
      } else {
        e.preventDefault();
        save();
      }
    }
  }

  function handleEditorInput() {
    detectCurrentLine();
    updateCurrentLineTags();
  }

  function handleEditorClick() {
    detectCurrentLine();
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
  <div class="header-hint" role="note" aria-label="Hint: Use Shift plus Enter for multiple wins">
    <span class="hint-text">💡 Use <kbd>Shift+Enter</kbd> for multiple wins</span>
  </div>
  
  <div class="logwin-input-row">
    <div
      bind:this={editorRef}
      class="logwin-input"
      contenteditable="true"
      on:keydown={handleEditorKeydown}
      on:input={handleEditorInput}
      on:click={handleEditorClick}
      role="textbox"
      tabindex="0"
      aria-label="Log your quiet win..."
    ></div>
  </div>
  
  <div class="tag-editor">
    <label class="tag-label">Tags for current line:</label>
    <input
      class="tag-input"
      type="text"
      placeholder="Tags auto-populate (edit as needed)..."
      bind:value={tagInput}
      on:input={handleTagInputChange}
      on:keydown={(e) => { if (e.key === 'Enter') addTag(); }}
    />
    {#if tagError}
      <span class="tag-error">{tagError}</span>
    {/if}
  </div>
  
  {#if showBanner}
    <div class="banner">
      {getEditorLines().length > 1 ? `${getEditorLines().length} wins logged!` : 'Win logged!'}
    </div>
  {/if}
  
  <div class="footer">
    <div class="footer-buttons">
      <button class="settings-btn-text" on:click={openSettings} title="Settings" aria-label="Open settings">
        <Settings size={16} />
        <span>Settings</span>
      </button>
      <button class="help-btn-text" on:click={openHelp} title="Help / Onboarding" aria-label="Open help and onboarding">
        <HelpCircle size={16} />
        <span>Help</span>
      </button>
    </div>
  </div>

  {#if showTagReview}
    <div class="modal-backdrop" aria-hidden="false">
      <div
        class="modal tag-review-modal"
        role="dialog"
        aria-modal="true"
        aria-labelledby="tag-review-title"
        aria-describedby="tag-review-desc"
        tabindex="-1"
        bind:this={modalRef}
        on:keydown={handleModalKeydown}
      >
        <h3 id="tag-review-title">Review Tags for Your Wins</h3>
        <p id="tag-review-desc">Edit tags for each win before saving. Separate tags with commas.</p>
        <ul class="tag-review-list">
          {#each reviewLines as line}
            <li>
              <div class="review-text">{line.text}</div>
              <input
                class="review-tag-input"
                type="text"
                bind:value={reviewTagsByLine[line.index]}
                on:input={(e) => {
                  reviewTagsByLine[line.index] = e.target.value.split(',').map(t => t.trim()).filter(Boolean);
                }}
                placeholder="Edit tags..."
              />
            </li>
          {/each}
        </ul>
        {#if reviewError}
          <div class="tag-error">{reviewError}</div>
        {/if}
        <div class="modal-actions">
          <button on:click={confirmTagReview}>Save All</button>
          <button on:click={cancelTagReview} class="cancel">Cancel</button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style>
  .header-hint {
    margin-bottom: 0.8rem;
    text-align: center;
  }
  
  .hint-text {
    font-size: 0.85rem;
    color: #666;
    font-style: italic;
  }
  
  .hint-text kbd {
    font-family: inherit;
    font-style: normal;
    background: #e8e8e8;
    padding: 0.1rem 0.4rem;
    border-radius: 3px;
    font-size: 0.75rem;
    border: 1px solid #ccc;
  }
  
  .tag-editor {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
    margin-bottom: 1rem;
  }
  
  .tag-label {
    font-size: 0.9rem;
    color: #666;
    font-weight: 500;
  }
  
  .tag-input {
    width: 100%;
    font-size: 1rem;
    border-radius: 8px;
    border: 1px solid #ccc;
    padding: 0.5rem 0.7rem;
    font-family: inherit;
  }
  
  .tag-input:focus {
    outline: none;
    border-color: var(--accent, #CC785C);
  }
  
  .tag-error {
    color: #a95e45;
    font-size: 0.92em;
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
    min-width: 400px;
    max-width: 600px;
    z-index: 9999;
  }

  .logwin-input-row {
    display: flex;
    align-items: flex-start;
    margin-bottom: 1.2rem;
  }

  .logwin-input {
    width: 100%;
    min-height: 120px;
    max-height: 400px;
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
    margin: 0.3rem 0;
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
    justify-content: flex-end;
    align-items: center;
    margin-top: 1rem;
  }

  .footer-buttons {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .settings-btn-text,
  .help-btn-text {
    font-size: 0.9rem;
    padding: 0.4rem 0.8rem;
    border-radius: 6px;
    border: 1px solid #ccc;
    background: #fff;
    color: #333;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    display: flex;
    align-items: center;
    gap: 0.4rem;
  }

  .settings-btn-text:focus-visible,
  .help-btn-text:focus-visible,
  .modal-actions button:focus-visible,
  .tag-input:focus-visible,
  .review-tag-input:focus-visible,
  .logwin-input:focus-visible {
    outline: 2px solid #a95e45;
    outline-offset: 2px;
  }

  .settings-btn-text:hover,
  .help-btn-text:hover {
    background: color-mix(in srgb, var(--accent, #CC785C) 80%, #fff 20%);
    border-color: var(--accent, #CC785C);
  }

  @keyframes fadein {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  .modal-backdrop {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0,0,0,0.25);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 2000;
  }
  
  .modal.tag-review-modal {
    background: #fff;
    border-radius: 12px;
    box-shadow: 0 4px 24px rgba(0,0,0,0.18);
    padding: 2rem 2.5rem;
    min-width: 340px;
    max-width: 90vw;
    display: flex;
    flex-direction: column;
    gap: 1.1rem;
  }
  
  .tag-review-list {
    list-style: none;
    padding: 0;
    margin: 0 0 1.2rem 0;
  }
  
  .tag-review-list li {
    margin-bottom: 1.1rem;
    padding-bottom: 0.7rem;
    border-bottom: 1px solid #eee;
  }
  
  .review-text {
    font-size: 1.08em;
    margin-bottom: 0.3em;
    color: #222;
  }
  
  .review-tag-input {
    width: 100%;
    font-size: 1rem;
    border-radius: 8px;
    border: 1px solid #ccc;
    padding: 0.5rem 0.7rem;
    font-family: inherit;
    margin-bottom: 0.2em;
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
</style>

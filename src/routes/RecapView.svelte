<script lang="ts">
  import { onMount } from 'svelte';
  import { getWins, type Win } from '../lib/tauri';
  let wins: Win[] = [];
  let filter = '7'; // '7' or '30'
  let filtered: Win[] = [];

  function applyFilter() {
    const days = parseInt(filter);
    const cutoff = new Date();
    cutoff.setDate(cutoff.getDate() - days + 1);
    filtered = wins.filter(win => {
      if (!win.date) return false;
      const d = new Date(win.date);
      return d >= cutoff;
    });
  }

  onMount(async () => {
    wins = await getWins();
    applyFilter();
  });

  $: if (filter && wins.length) applyFilter();
</script>

<main>
  <h2>Recap</h2>
  <div style="margin-bottom:1.2rem">
    <label>Show last
      <select bind:value={filter}>
        <option value="7">7 days</option>
        <option value="30">30 days</option>
      </select>
      of wins
    </label>
  </div>
  {#if filtered.length === 0}
    <div>No wins in this period.</div>
  {:else}
    <ul>
      {#each filtered as win}
        <li>
          <div><strong>{win.date}</strong>: {win.text}</div>
          <div style="font-size:0.95em;color:#a95e45;">Tags: {win.tags}</div>
        </li>
      {/each}
    </ul>
  {/if}
</main>

<style>
ul {
main {
  max-width: 520px;
  margin: 2rem auto;
  background: var(--background, #fff);
  color: var(--text, #222);
  border-radius: 16px;
  box-shadow: 0 4px 24px rgba(0,0,0,0.08);
  padding: 2rem;
  font-family: inherit;
}
select {
  margin: 0 0.5em;
  font-size: 1em;
  border-radius: 6px;
  border: 1px solid #bbb;
  padding: 0.2em 0.7em;
}
ul {
  list-style: none;
  padding: 0;
}
li {
  margin-bottom: 1.2em;
  padding-bottom: 0.7em;
  border-bottom: 1px solid #eee;
}
</style>

<script lang="ts">
  import { onMount } from 'svelte';
  import { getWins, type Win } from '../lib/tauri';
    import Settings from 'lucide-svelte/icons/settings';
  import { goto } from '$app/navigation';
  let wins: Win[] = [];

  onMount(async () => {
    wins = await getWins();
  });

  function openSettings() {
    goto('/Settings');
  }
</script>

<main>
  <div class="header">
    <h1>Quiet Wins Log</h1>
    <div class="header-actions">
      <button class="graph-btn" on:click={() => goto('/GraphView')} title="View Tag Graph">Graph</button>
      <button class="settings-btn" on:click={openSettings} title="Settings">
        <Settings class="settings-icon" />
      </button>
    </div>
  </div>
  {#each wins as win (win.id)}
    <section>
      <div>{win.date}</div>
      <div>{win.text}</div>
      <div>{win.tags}</div>
    </section>
  {/each}
</main>

<style>
main {
  font-family: 'SF Pro', 'San Francisco', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Arial, sans-serif;
  padding: 2rem;
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
  gap: 1rem;
}
.graph-btn {
  background: #007aff;
  color: #fff;
  border: none;
  border-radius: 6px;
  padding: 0.4rem 1.2rem;
  font-size: 1rem;
  cursor: pointer;
  transition: background 0.2s;
}
.graph-btn:hover {
  background: #005bb5;
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
section {
  margin-bottom: 1.5rem;
  padding: 1rem;
  border-radius: 8px;
  background: rgba(0,0,0,0.03);
}
</style>

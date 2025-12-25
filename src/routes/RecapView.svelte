<script lang="ts">
  // Placeholder for weekly recap logic
  import type { Win } from '../lib/tauri';
  export let wins: Win[] = [];
</script>

<main>
  <h2>Weekly Recap</h2>
  <div>Total Wins: {wins.length}</div>
  <div>
    Most Frequent Tags:
    <ul>
      {#each Array.from(new Set(wins.flatMap(win => win.tags.split(',')))) as tag}
        <li>{tag}</li>
      {/each}
    </ul>
  </div>
  <div class="summary">
    <em>Great job celebrating your quiet wins this week!</em>
  </div>
</main>

<style>
main {
  font-family: 'SF Pro', 'San Francisco', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Arial, sans-serif;
  padding: 2rem;
}
.summary {
  margin-top: 1.5rem;
  font-size: 1.1rem;
  color: #007aff;
}
ul {
  padding-left: 1.2rem;
}
</style>

<script lang="ts">
  import { onMount } from 'svelte';
  import { getWins, type Win } from '../lib/tauri';
  let wins: Win[] = [];
  let filter = '7'; // '7' or '30'
  let filtered: Win[] = [];

  type Summary = {
    total: number;
    perDay: number;
    topTags: { tag: string; count: number }[];
    periodLabel: string;
  };

  let summary: Summary = { total: 0, perDay: 0, topTags: [], periodLabel: '' };

  function parseTags(raw: string | undefined): string[] {
    if (!raw) return [];
    return raw
      .split(/[\,;\[\]\"\|]/)
      .map((t) => t.trim())
      .filter(Boolean);
  }

  function applyFilter() {
    const days = parseInt(filter);
    const now = new Date();
    const cutoff = new Date();
    cutoff.setDate(now.getDate() - days + 1);
    filtered = wins.filter((win) => {
      if (!win.date) return false;
      const d = new Date(win.date);
      return d >= cutoff;
    });
    buildSummary(days, cutoff, now);
  }

  function buildSummary(days: number, start: Date, end: Date) {
    const total = filtered.length;
    const perDay = days > 0 ? Number((total / days).toFixed(2)) : 0;
    const tagCounts = new Map<string, number>();
    filtered.forEach((win) => {
      parseTags(win.tags).forEach((t) => {
        tagCounts.set(t, (tagCounts.get(t) || 0) + 1);
      });
    });
    const topTags = Array.from(tagCounts.entries())
      .sort((a, b) => b[1] - a[1])
      .slice(0, 5)
      .map(([tag, count]) => ({ tag, count }));
    const periodLabel = `${start.toISOString().slice(0, 10)} → ${end.toISOString().slice(0, 10)}`;
    summary = { total, perDay, topTags, periodLabel };
  }

  onMount(async () => {
    wins = await getWins();
    applyFilter();
  });

  $: if (filter && wins.length) applyFilter();
</script>

<main>
  <h2>Recap</h2>
  <div class="controls">
    <label for="recap-range">Show last</label>
    <select id="recap-range" bind:value={filter} aria-label="Select recap range">
      <option value="7">7 days</option>
      <option value="30">30 days</option>
    </select>
    <span>of wins</span>
  </div>

  <section class="summary">
    <div class="stat">
      <div class="label">Total wins</div>
      <div class="value">{summary.total}</div>
    </div>
    <div class="stat">
      <div class="label">Avg per day</div>
      <div class="value">{summary.perDay}</div>
    </div>
    <div class="stat period">
      <div class="label">Period</div>
      <div class="value small">{summary.periodLabel}</div>
    </div>
  </section>

  <section class="top-tags" aria-label="Top tags">
    <h3>Top tags</h3>
    {#if summary.topTags.length === 0}
      <div class="empty">No tags for this period.</div>
    {:else}
      <ul>
        {#each summary.topTags as t}
          <li>{t.tag} <span class="count">×{t.count}</span></li>
        {/each}
      </ul>
    {/if}
  </section>

  <section class="win-list" aria-label="Wins for selected range">
    <h3>Wins</h3>
    {#if filtered.length === 0}
      <div class="empty">No wins in this period.</div>
    {:else}
      <ul>
        {#each filtered as win}
          <li>
            <div class="win-date">{win.date}</div>
            <div class="win-text">{win.text}</div>
            {#if win.tags}
              <div class="win-tags">Tags: {win.tags}</div>
            {/if}
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</main>

<style>
main {
  max-width: 720px;
  margin: 2rem auto;
  background: var(--background, #fff);
  color: var(--text, #222);
  border-radius: 16px;
  box-shadow: 0 4px 24px rgba(0,0,0,0.08);
  padding: 2rem;
  font-family: inherit;
}

.controls {
  margin-bottom: 1.2rem;
}

select {
  margin: 0 0.5em;
  font-size: 1em;
  border-radius: 6px;
  border: 1px solid #bbb;
  padding: 0.2em 0.7em;
}

.summary {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat {
  background: #f9f7f6;
  border: 1px solid #eee;
  border-radius: 10px;
  padding: 1rem;
  box-shadow: 0 1px 4px rgba(0,0,0,0.04);
}

.stat .label {
  font-size: 0.95em;
  color: #777;
  margin-bottom: 0.3rem;
}

.stat .value {
  font-size: 1.6em;
  font-weight: 600;
  color: #a95e45;
}

.stat.period .value {
  font-size: 1em;
  font-weight: 500;
  color: #444;
}

.stat.period .value.small {
  font-size: 0.95em;
  color: #666;
}

.top-tags {
  margin-bottom: 1.5rem;
}

.top-tags ul {
  list-style: none;
  padding: 0;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 0.6rem;
}

.top-tags li {
  background: #fff7f3;
  border: 1px solid #f0d7c7;
  border-radius: 8px;
  padding: 0.6rem 0.8rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.top-tags .count {
  color: #a95e45;
  font-weight: 600;
}

.win-list ul {
  list-style: none;
  padding: 0;
}

.win-list li {
  margin-bottom: 1.1rem;
  padding: 0.8rem 1rem;
  background: #fff;
  border-radius: 8px;
  box-shadow: 0 1px 4px rgba(0,0,0,0.05);
}

.win-date {
  font-size: 0.95em;
  color: #777;
  margin-bottom: 0.2rem;
}

.win-text {
  margin-bottom: 0.3rem;
}

.win-tags {
  font-size: 0.95em;
  color: #a95e45;
}

.empty {
  color: #777;
  padding: 0.6rem 0;
}
</style>

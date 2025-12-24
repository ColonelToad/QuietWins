<script>
  // Placeholder for win graph logic
  export let categoryCounts = {};
</script>

<main>
  <h2>Win Categories (This Month)</h2>
  <div class="bar-chart">
    {#each Object.entries(categoryCounts) as [cat, count]}
      <div class="bar">
        <span>{cat}</span>
        <div class="bar-inner" style="width: {count * 20}px">{count}</div>
      </div>
    {/each}
  </div>
</main>

<style>
main {
  font-family: 'SF Pro', 'San Francisco', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Arial, sans-serif;
  padding: 2rem;
}
.bar-chart {
  margin-top: 1rem;
}
.bar {
  display: flex;
  align-items: center;
  margin-bottom: 0.8rem;
}
.bar-inner {
  height: 24px;
  background: #007aff;
  color: #fff;
  border-radius: 6px;
  margin-left: 1rem;
  padding: 0 0.8rem;
  display: flex;
  align-items: center;
}
span {
  min-width: 100px;
  font-weight: 500;
}
</style>

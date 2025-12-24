<script>
  // Placeholder for log view logic
  export let wins = [];
</script>

<main>
  <h1>Quiet Wins Log</h1>
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
section {
  margin-bottom: 1.5rem;
  padding: 1rem;
  border-radius: 8px;
  background: rgba(0,0,0,0.03);
}
</style>

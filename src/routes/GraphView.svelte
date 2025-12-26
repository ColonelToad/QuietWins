<script lang="ts">
  import { onMount } from 'svelte';
  import { getTagGraph, getWins, type Win } from '../lib/tauri';
  import { Network, DataSet, Node, Edge } from 'vis-network/standalone';
  import { goto } from '$app/navigation';
  import { save } from '@tauri-apps/api/dialog';
  import { writeBinaryFile, writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';

  let networkContainer: HTMLDivElement;
  let network: Network;
  let tagGraph: { nodes: string[]; edges: [string, string][] } = { nodes: [], edges: [] };
  let selectedTag: string | null = null;
  let wins: Win[] = [];
  let filteredWins: Win[] = [];
  let nodesDS: DataSet<Node>;
  let edgesDS: DataSet<Edge>;

  async function exportAsImage() {
    if (!network) return;
    const canvasEl = (network as any)?.body?.canvas?.frame?.canvas as HTMLCanvasElement | undefined;
    if (!canvasEl) return;
    const dataUrl: string = canvasEl.toDataURL('image/png');
    // Convert base64 to binary
    const base64 = dataUrl.split(',')[1];
    const binary = Uint8Array.from(atob(base64), c => c.charCodeAt(0));
    // Remember last dir
    let lastDir = localStorage.getItem('qw-last-export-dir') || undefined;
    const filePath = await save({
      defaultPath: lastDir ? `${lastDir}/tag-graph.png` : 'tag-graph.png',
      filters: [{ name: 'PNG Image', extensions: ['png'] }]
    });
    if (filePath) {
      // Save file
      await writeBinaryFile({ path: filePath, contents: binary });
      // Remember dir
      const dir = filePath.substring(0, filePath.lastIndexOf('/'));
      localStorage.setItem('qw-last-export-dir', dir);
    }
  }

  async function exportAsJSON() {
    const json: string = JSON.stringify(tagGraph, null, 2);
    let lastDir = localStorage.getItem('qw-last-export-dir') || undefined;
    const filePath = await save({
      defaultPath: lastDir ? `${lastDir}/tag-graph.json` : 'tag-graph.json',
      filters: [{ name: 'JSON', extensions: ['json'] }]
    });
    if (filePath) {
      await writeTextFile({ path: filePath, contents: json });
      const dir = filePath.substring(0, filePath.lastIndexOf('/'));
      localStorage.setItem('qw-last-export-dir', dir);
    }
  }



  onMount(async () => {
    tagGraph = await getTagGraph();
    wins = await getWins();
    nodesDS = new DataSet<Node>(
      tagGraph.nodes.map((tag) => ({ id: tag, label: tag }))
    );
    edgesDS = new DataSet<Edge>(
      tagGraph.edges.map(([from, to]) => ({ from, to }))
    );
    network = new Network(networkContainer, { nodes: nodesDS, edges: edgesDS }, {
      nodes: { shape: 'dot', size: 18, font: { size: 16 } },
      edges: { color: '#aaa', width: 2, smooth: true },
      physics: { stabilization: { fit: true } },  // Enable fit on stabilization
      autoResize: true,
      interaction: { hover: true, navigationButtons: true, selectable: true }
    });

    network.on('click', function(params) {
      if (params.nodes.length > 0) {
        selectedTag = params.nodes[0] as string;
        filterGraph(selectedTag);
        filterWins(selectedTag);
      } else {
        selectedTag = null;
        clearFilter();
        filteredWins = [];
      }
    });
  });

  function filterWins(tag: string | null) {
    if (!tag) {
      filteredWins = [];
      return;
    }
    filteredWins = wins.filter(win => win.tags && win.tags.split(/[,\[\]"]+/).map(t => t.trim()).filter(Boolean).includes(tag));
  }

  function filterGraph(tag: string | null) {
    if (!tag) return;
    // Find neighbors
    const neighbors = new Set<string>();
    tagGraph.edges.forEach(([from, to]) => {
      if (from === tag) neighbors.add(to);
      if (to === tag) neighbors.add(from);
    });
    neighbors.add(tag);
    // Update node/edge styles
    nodesDS.forEach((node) => {
      nodesDS.update({ id: node.id, color: neighbors.has(node.id as string) ? undefined : { background: '#eee', border: '#ccc' }, font: { color: neighbors.has(node.id as string) ? '#222' : '#bbb' } });
    });
    edgesDS.forEach((edge) => {
      const show = neighbors.has(edge.from as string) && neighbors.has(edge.to as string);
      edgesDS.update({ id: edge.id, color: show ? '#aaa' : '#eee' });
    });
  }

  function clearFilter() {
    // Restore all node/edge styles
    nodesDS.forEach((node) => {
      nodesDS.update({ id: node.id, color: undefined, font: { color: '#222' } });
    });
    edgesDS.forEach((edge) => {
      edgesDS.update({ id: edge.id, color: '#aaa' });
    });
  }

  function openLogForTag() {
    if (selectedTag) goto(`/LogView?tag=${encodeURIComponent(selectedTag)}`);
  }
</script>


<main>
  <h2>Tag Graph</h2>
  <div class="export-bar">
    <button on:click={exportAsImage}>Export as Image</button>
    <button on:click={exportAsJSON}>Export as JSON</button>
  </div>
  <div bind:this={networkContainer} class="graph-container"></div>
  {#if selectedTag}
    <div class="tag-details">
      <strong>Selected Tag:</strong> {selectedTag}
      <button on:click={openLogForTag}>Show Wins</button>
      <button on:click={() => { selectedTag = null; clearFilter(); filteredWins = []; }}>Clear Filter</button>
      <div class="win-list">
        <h3>Wins with "{selectedTag}"</h3>
        {#if filteredWins.length === 0}
          <div class="empty">No wins found for this tag.</div>
        {:else}
          <ul>
            {#each filteredWins as win}
              <li>
                <div class="win-date">{win.date}</div>
                <div class="win-text">{win.text}</div>
                <div class="win-tags">Tags: {win.tags}</div>
              </li>
            {/each}
          </ul>
        {/if}
      </div>
    </div>
  {/if}
</main>


<style>
main {
  font-family: 'SF Pro', 'San Francisco', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Arial, sans-serif;
  padding: 2rem;
}
.graph-container {
  width: 100%;
  height: 500px;
  border: 1px solid #ccc;
  border-radius: 10px;
  margin-bottom: 1.5rem;
  background: #fff;
}
.export-bar {
  margin-bottom: 1rem;
}
.export-bar button {
  margin-right: 1rem;
  padding: 0.4rem 1.2rem;
  border-radius: 6px;
  border: none;
  background: #007aff;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}
.export-bar button:hover {
  background: #005bb5;
}
.tag-details {
  margin-top: 1rem;
  padding: 1rem;
  background: #f7f7f7;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.04);
  max-width: 600px;
}
.win-list {
  margin-top: 1.2rem;
}
.win-list ul {
  list-style: none;
  padding: 0;
}
.win-list li {
  margin-bottom: 1.1rem;
  padding: 0.7rem 1rem;
  background: #fff;
  border-radius: 6px;
  box-shadow: 0 1px 4px rgba(0,0,0,0.04);
}
.win-date {
  font-size: 0.95em;
  color: #888;
}
.win-text {
  font-size: 1.08em;
  margin: 0.2em 0 0.3em 0;
}
.win-tags {
  font-size: 0.92em;
  color: #666;
}
.empty {
  color: #999;
  font-style: italic;
  margin-top: 0.5em;
}
button {
  margin-left: 1rem;
  padding: 0.4rem 1.2rem;
  border-radius: 6px;
  border: none;
  background: #007aff;
  color: #fff;
  cursor: pointer;
  transition: background 0.2s;
}
button:hover {
  background: #005bb5;
}
</style>


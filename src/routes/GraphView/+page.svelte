<script lang="ts">
  import { onMount } from 'svelte';
  import { getTagGraph, getWins, type Win } from '../../lib/tauri';
  import * as d3 from 'd3';
  import { goto } from '$app/navigation';

  let networkContainer: HTMLDivElement;
  let svgEl: SVGSVGElement;
  let tagGraph: { nodes: string[]; edges: [string, string][] } = { nodes: [], edges: [] };
  let selectedTag: string | null = null;
  let wins: Win[] = [];
  let filteredWins: Win[] = [];
  // D3 doesn't need DataSet, will use arrays
  let errorMsg: string | null = null;

  function exportAsImage(): void {
    if (!svgEl) return;
    const serializer = new XMLSerializer();
    const svgString = serializer.serializeToString(svgEl);
    const canvas = document.createElement('canvas');
    const bbox = svgEl.getBBox();
    canvas.width = bbox.width + 40;
    canvas.height = bbox.height + 40;
    const ctx = canvas.getContext('2d');
    const img = new window.Image();
    const svg64 = btoa(unescape(encodeURIComponent(svgString)));
    const image64 = 'data:image/svg+xml;base64,' + svg64;
    img.onload = function () {
      ctx?.clearRect(0, 0, canvas.width, canvas.height);
      ctx?.drawImage(img, 20, 20);
      const dataUrl = canvas.toDataURL('image/png');
      const link = document.createElement('a');
      link.href = dataUrl;
      link.download = 'tag-graph.png';
      link.click();
    };
    img.src = image64;
  }

  function exportAsJSON(): void {
    const json: string = JSON.stringify(tagGraph, null, 2);
    const blob: Blob = new Blob([json], { type: 'application/json' });
    const url: string = URL.createObjectURL(blob);
    const link: HTMLAnchorElement = document.createElement('a');
    link.href = url;
    link.download = 'tag-graph.json';
    link.click();
    setTimeout(() => URL.revokeObjectURL(url), 1000);
  }

  onMount(async () => {
    try {
      tagGraph = await getTagGraph();
    } catch (err) {
      console.error('GraphView getTagGraph error:', err);
      errorMsg = `Failed to load tag graph: ${typeof err === 'object' && err !== null && 'message' in err ? (err as { message?: string }).message ?? String(err) : String(err)}`;
      return;
    }
    try {
      wins = await getWins();
    } catch (err) {
      console.error('GraphView getWins error:', err);
      errorMsg = `Failed to load wins: ${typeof err === 'object' && err !== null && 'message' in err ? (err as { message?: string }).message ?? String(err) : String(err)}`;
      return;
    }
    // D3 rendering
    renderD3Graph();
  });

  function renderD3Graph() {
    if (!networkContainer) return;
    // Remove previous SVG if any
    networkContainer.innerHTML = '';
    const width = networkContainer.clientWidth || 800;
    const height = 500;
    const svg = d3.select(networkContainer)
      .append('svg')
      .attr('width', width)
      .attr('height', height)
      .attr('style', 'background: #fff; border-radius: 10px; display: block; margin: 0 auto;');
    svgEl = svg.node() as SVGSVGElement;

    // Prepare data
    const nodes: { id: string; x?: number; y?: number }[] = tagGraph.nodes.map(tag => ({ id: tag }));
    const links: { source: string; target: string }[] = tagGraph.edges.map(([source, target]) => ({ source, target }));

    const simulation = d3.forceSimulation(nodes)
      .force('link', d3.forceLink(links).id((d: { id: string }) => d.id).distance(80))
      .force('charge', d3.forceManyBody().strength(-220))
      .force('center', d3.forceCenter(width / 2, height / 2));

    // Draw links
    const link = svg.append('g')
      .attr('stroke', '#aaa')
      .attr('stroke-width', 2)
      .selectAll('line')
      .data(links)
      .enter().append('line');

    // Draw nodes
    const node = svg.append('g')
      .attr('stroke', '#fff')
      .attr('stroke-width', 1.5)
      .selectAll('circle')
      .data(nodes)
      .enter().append('circle')
      .attr('r', 18)
      .attr('fill', (d: { id: string }) => selectedTag === d.id ? '#007aff' : '#f7b267')
      .attr('cursor', 'pointer')
      .on('click', (event: MouseEvent, d: { id: string }) => {
        selectedTag = d.id;
        filterGraph(selectedTag);
        filterWins(selectedTag);
        renderD3Graph();
      });

    // Draw labels
    const label = svg.append('g')
      .selectAll('text')
      .data(nodes)
      .enter().append('text')
      .text((d: { id: string }) => d.id)
      .attr('font-size', 14)
      .attr('text-anchor', 'middle')
      .attr('dy', 5)
      .attr('pointer-events', 'none');

    simulation.on('tick', () => {
      link
        .attr('x1', (d: any) => (d.source as { x: number }).x)
        .attr('y1', (d: any) => (d.source as { y: number }).y)
        .attr('x2', (d: any) => (d.target as { x: number }).x)
        .attr('y2', (d: any) => (d.target as { y: number }).y);
      node
        .attr('cx', (d: { x: number }) => d.x)
        .attr('cy', (d: { y: number }) => d.y);
      label
        .attr('x', (d: { x: number }) => d.x)
        .attr('y', (d: { y: number }) => d.y);
    });

    // Recenter on window resize
    window.addEventListener('resize', recenterGraph);
    function recenterGraph() {
      const w = networkContainer.clientWidth || 800;
      const h = 500;
      svg.attr('width', w).attr('height', h);
      simulation.force('center', d3.forceCenter(w / 2, h / 2));
      simulation.alpha(0.3).restart();
    }
  }

  function filterWins(tag: string | null) {
    if (!tag) {
      filteredWins = [];
      return;
    }
    filteredWins = wins.filter(win => win.tags && win.tags.split(/[,\[\]"]+/).map(t => t.trim()).filter(Boolean).includes(tag));
  }

  function filterGraph(tag: string | null) {
    // D3: just re-render with new selectedTag
    renderD3Graph();
  }

  function clearFilter() {
    selectedTag = null;
    filteredWins = [];
    renderD3Graph();
  }

</script>

<main>
  <div class="export-bar">
    <button on:click={exportAsImage}>Export as Image</button>
    <button on:click={exportAsJSON}>Export as JSON</button>
    {#if selectedTag}
      <button on:click={clearFilter}>Clear Tag Filter</button>
    {/if}
  </div>
  <div bind:this={networkContainer} class="graph-container"></div>
  {#if errorMsg}
    <div class="error">{errorMsg}</div>
  {/if}
  {#if selectedTag}
    <div class="tag-details">
      <h2>Tag: {selectedTag}</h2>
      <div class="win-list">
        <ul>
          {#each filteredWins as win}
            <li>
              <div class="win-date">{win.date}</div>
              <div>{win.text}</div>
            </li>
          {/each}
        </ul>
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
.error {
  color: #b00020;
  background: #ffeaea;
  border: 1px solid #b00020;
  padding: 1rem;
  border-radius: 8px;
  margin-bottom: 1.5rem;
  font-size: 1.1rem;
}
</style>

export interface TagGraph {
  nodes: string[];
  edges: [string, string][];
}


export async function getTagGraph(): Promise<TagGraph> {
  const { invoke } = await import('@tauri-apps/api/core');
  return await invoke('get_tag_graph');
}

export interface Win {
  id: number;
  date: string;
  text: string;
  tags: string;
}


export async function addWin(date: string, text: string, tags: string) {
  const { invoke } = await import('@tauri-apps/api/core');
  return await invoke('add_win', { date, text, tags });
}

export async function getWins(): Promise<Win[]> {
  const { invoke } = await import('@tauri-apps/api/core');
  return await invoke('get_wins');
}

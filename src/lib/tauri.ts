export interface TagGraph {
  nodes: string[];
  edges: [string, string][];
}

export async function getTagGraph(): Promise<TagGraph> {
  return await invoke('get_tag_graph');
}


import { invoke } from '@tauri-apps/api/core';

export interface Win {
  id: number;
  date: string;
  text: string;
  tags: string;
}

export async function addWin(date: string, text: string, tags: string) {
  return await invoke('add_win', { date, text, tags });
}

export async function getWins(): Promise<Win[]> {
  return await invoke('get_wins');
}

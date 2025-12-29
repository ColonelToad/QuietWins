import { invoke } from '@tauri-apps/api/core';

export interface TagGraph {
  nodes: string[];
  edges: [string, string][];
}


export async function getTagGraph(): Promise<TagGraph> {
  return await invoke('get_tag_graph');
}

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

export async function suggestTagsForText(text: string): Promise<string[]> {
  return await invoke('suggest_tags_for_text', { text });
}

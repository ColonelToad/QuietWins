import { invoke } from '@tauri-apps/api/core';

export interface WinWithChain {
  id: number;
  date: string;
  text: string;
  tags: string;
  created_at: number;
  chain_id: number;
}

export async function getWinsWithChains(): Promise<WinWithChain[]> {
  return await invoke('get_wins_with_chains');
}

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


export async function addWin(win: { date: string; text: string; tags: string }) {
  return await invoke('add_win', { date: win.date, text: win.text, tags: win.tags });
}

export async function getWins(): Promise<Win[]> {
  return await invoke('get_wins');
}

export async function getDeletedWins(): Promise<Win[]> {
  return await invoke('get_deleted_wins');
}

export async function updateWin(win: { id: number; date: string; text: string; tags: string }) {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke('update_win', { ...win });
}

export async function deleteWin(id: number) {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke('delete_win', { id });
}

export async function restoreWin(id: number) {
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke('restore_win', { id });
}

export async function suggestTagsForText(text: string): Promise<string[]> {
  try {
    const res = await invoke('suggest_tags_for_text', { text });
    if (Array.isArray(res)) return res as string[];
    return [];
  } catch (e) {
    console.warn('suggestTagsForText failed:', e);
    return [];
  }
}

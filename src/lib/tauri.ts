import { invoke } from '@tauri-apps/api/tauri';

export async function addWin(date: string, text: string, tags: string) {
  return await invoke('add_win', { date, text, tags });
}

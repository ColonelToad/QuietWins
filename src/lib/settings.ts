import { writable } from 'svelte/store';

export interface Settings {
  theme: string;
  icon: string;
  notifTime: string;
  notifSound: boolean;
  shortcut: string;
  font: string;
  autoTag: boolean;
  privacyLock: boolean;
  startup: boolean;
}

const defaultSettings: Settings = {
  theme: 'warm',
  icon: 'warm',
  notifTime: '20:00',
  notifSound: true,
  shortcut: 'Cmd+Alt+Shift+W',
  font: 'Garamond',
  autoTag: true,
  privacyLock: false,
  startup: true,
};


// Load notification time from backend if available
async function loadNotifTimeFromBackend(): Promise<string> {
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    const time = await invoke<string>('get_notif_time');
    if (typeof time === 'string' && /^\d{2}:\d{2}$/.test(time)) {
      return time;
    }
  } catch {}
  return defaultSettings.notifTime;
}

async function loadSettings(): Promise<Settings> {
  let notifTime = defaultSettings.notifTime;
  try {
    notifTime = await loadNotifTimeFromBackend();
  } catch {}
  try {
    const raw = localStorage.getItem('qw-settings');
    if (raw) return { ...defaultSettings, ...JSON.parse(raw), notifTime };
  } catch {}
  return { ...defaultSettings, notifTime };
}

// Use an async store initializer
const settingsStore = writable<Settings>(defaultSettings);
loadSettings().then(settingsStore.set);

settingsStore.subscribe((val) => {
  localStorage.setItem('qw-settings', JSON.stringify(val));
  // Sync notifTime to backend
  import('@tauri-apps/api/core').then(({ invoke }) => {
    invoke('set_notif_time', { notif_time: val.notifTime });
  });
});

export const settings = settingsStore;

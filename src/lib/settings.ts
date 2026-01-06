import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export interface Settings {
  theme: string;
  icon: string;
  notifTime: string;
  notifSound: boolean;
  notifEnabled: boolean;
  weeklyRecap: boolean;
  notifFrequency: 'daily' | 'off';
  dailyMessage: string;
  weeklyMessage: string;
  shortcut: string;
  font: string;
  autoTag: boolean;
  privacyLock: boolean;
  startup: boolean;
  telemetryEnabled: boolean;
}

const defaultSettings: Settings = {
  theme: 'warm',
  icon: 'warm',
  notifTime: '20:00',
  notifSound: true,
  notifEnabled: true,
  weeklyRecap: true,
  notifFrequency: 'daily',
  dailyMessage: "Don't forget to log your quiet win today!",
  weeklyMessage: '',
  shortcut: 'Cmd+Alt+Shift+W',
  font: 'Garamond',
  autoTag: true,
  privacyLock: false,
  startup: true,
  telemetryEnabled: false,
};


// Load notification time from backend if available
async function loadNotifTimeFromBackend(): Promise<string> {
  if (typeof window === 'undefined') return defaultSettings.notifTime;
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


const settingsStore = writable<Settings>(defaultSettings);

if (browser) {
  loadSettings().then(settingsStore.set);

  settingsStore.subscribe((val) => {
    localStorage.setItem('qw-settings', JSON.stringify(val));
    syncSettingsToBackend(val);
  });
}

async function syncSettingsToBackend(val: Settings) {
  if (typeof window === 'undefined') return;
  // Only run in browser/tauri context
  try {
    const { invoke } = await import('@tauri-apps/api/core');
    await invoke('set_notif_time', { notif_time: val.notifTime });
  } catch {}
  try {
    const { writeTextFile, BaseDirectory } = await import('@tauri-apps/plugin-fs');
    const appDataPath = BaseDirectory.AppData;
    await writeTextFile(`${appDataPath}/settings.json`, JSON.stringify(val));
  } catch {}
}

export const settings = settingsStore;

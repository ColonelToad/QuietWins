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

function loadSettings(): Settings {
  try {
    const raw = localStorage.getItem('qw-settings');
    if (raw) return { ...defaultSettings, ...JSON.parse(raw) };
  } catch {}
  return defaultSettings;
}

export const settings = writable<Settings>(loadSettings());

settings.subscribe((val) => {
  localStorage.setItem('qw-settings', JSON.stringify(val));
});

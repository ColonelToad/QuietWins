// userTagPrefs.ts - Store and retrieve user tag preferences in localStorage

const STORAGE_KEY = 'qw-user-tag-prefs';

export type TagPrefMap = Record<string, string[]>; // keyword/phrase -> tags[]

export function getUserTagPrefs(): TagPrefMap {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (raw) return JSON.parse(raw);
  } catch {}
  return {};
}

export function saveUserTagPrefs(prefs: TagPrefMap) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(prefs));
}

export function addUserTagPref(keyword: string, tags: string[]) {
  const prefs = getUserTagPrefs();
  prefs[keyword.toLowerCase()] = tags;
  saveUserTagPrefs(prefs);
}

export function getTagsForKeyword(keyword: string): string[] | undefined {
  const prefs = getUserTagPrefs();
  return prefs[keyword.toLowerCase()];
}

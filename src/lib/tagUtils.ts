// tagUtils.ts - Tag normalization and spellcheck helpers

// Normalize a tag: lowercase and trim whitespace
export function normalizeTag(tag: string): string {
  return tag.trim().toLowerCase();
}

// Remove duplicates (case-insensitive)
export function uniqueTags(tags: string[]): string[] {
  const seen = new Set<string>();
  return tags.filter(tag => {
    const norm = normalizeTag(tag);
    if (seen.has(norm)) return false;
    seen.add(norm);
    return true;
  });
}

// Simple spellcheck: return closest tag from a list, or null
export function didYouMean(input: string, candidates: string[]): string | null {
  const normInput = normalizeTag(input);
  let minDist = Infinity;
  let best: string | null = null;
  for (const cand of candidates) {
    const dist = levenshtein(normInput, normalizeTag(cand));
    if (dist < minDist && dist <= 2) { // allow up to 2 edits
      minDist = dist;
      best = cand;
    }
  }
  return best;
}

// Levenshtein distance
function levenshtein(a: string, b: string): number {
  const dp = Array.from({ length: a.length + 1 }, () => new Array(b.length + 1).fill(0));
  for (let i = 0; i <= a.length; i++) dp[i][0] = i;
  for (let j = 0; j <= b.length; j++) dp[0][j] = j;
  for (let i = 1; i <= a.length; i++) {
    for (let j = 1; j <= b.length; j++) {
      if (a[i - 1] === b[j - 1]) dp[i][j] = dp[i - 1][j - 1];
      else dp[i][j] = 1 + Math.min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1]);
    }
  }
  return dp[a.length][b.length];
}

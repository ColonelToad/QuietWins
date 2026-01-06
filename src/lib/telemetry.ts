import { get } from 'svelte/store';
import { settings } from './settings';

export type TelemetryEvent = {
  name: string;
  data?: Record<string, unknown>;
};

// No-op logger respecting opt-in toggle.
export async function logEvent(event: TelemetryEvent): Promise<void> {
  const { telemetryEnabled } = get(settings);
  if (!telemetryEnabled) return;
  console.log('[telemetry][noop]', event.name, event.data ?? {});
}

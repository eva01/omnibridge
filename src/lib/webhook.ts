import { Store } from '@tauri-apps/plugin-store';

const STORE_FILE = 'omnibridge.webhooks.json';
const KEY_CONFIGS = 'configs';

export type WebhookMethod = 'POST' | 'PUT' | 'PATCH';

export interface WebhookConfig {
  enabled: boolean;
  url: string;
  method: WebhookMethod;
  throttle_ms: number;
  include_raw: boolean;
  custom_headers: string; // newline-separated "Key: Value"
}

export interface WebhookStats {
  sent: number;
  failed: number;
  last_status: string | null;
  last_sent_at: number | null;
  last_error: string | null;
}

export interface WebhookPayload {
  timestamp: number;
  port: string;
  device_class?: string;
  protocol: string;
  confidence: number;
  fields: Record<
    string,
    { value: string | number | boolean; unit?: string; updated_at: number }
  >;
  raw_line?: { ascii: string; hex: string; timestamp: number };
}

export function defaultConfig(): WebhookConfig {
  return {
    enabled: false,
    url: '',
    method: 'POST',
    throttle_ms: 1000,
    include_raw: false,
    custom_headers: '',
  };
}

export function defaultStats(): WebhookStats {
  return {
    sent: 0,
    failed: 0,
    last_status: null,
    last_sent_at: null,
    last_error: null,
  };
}

let _store: Store | null = null;
async function getStore(): Promise<Store> {
  if (!_store) _store = await Store.load(STORE_FILE);
  return _store;
}

async function getAllConfigs(): Promise<Record<string, WebhookConfig>> {
  try {
    const store = await getStore();
    return (await store.get<Record<string, WebhookConfig>>(KEY_CONFIGS)) ?? {};
  } catch {
    return {};
  }
}

export async function getConfig(fingerprint: string): Promise<WebhookConfig | null> {
  const all = await getAllConfigs();
  return all[fingerprint] ?? null;
}

export async function saveConfig(fingerprint: string, config: WebhookConfig): Promise<void> {
  const store = await getStore();
  const all = await getAllConfigs();
  all[fingerprint] = config;
  await store.set(KEY_CONFIGS, all);
  await store.save();
}

export async function deleteConfig(fingerprint: string): Promise<void> {
  const store = await getStore();
  const all = await getAllConfigs();
  delete all[fingerprint];
  await store.set(KEY_CONFIGS, all);
  await store.save();
}

export function parseHeaders(raw: string): Record<string, string> {
  const headers: Record<string, string> = {};
  for (const line of raw.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (!trimmed || trimmed.startsWith('#')) continue;
    const colonIdx = trimmed.indexOf(':');
    if (colonIdx < 0) continue;
    const key = trimmed.slice(0, colonIdx).trim();
    const value = trimmed.slice(colonIdx + 1).trim();
    if (key) headers[key] = value;
  }
  return headers;
}

export interface SendResult {
  ok: boolean;
  status: number | null;
  error?: string;
  bodyPreview?: string;
}

export async function sendWebhook(
  config: WebhookConfig,
  payload: WebhookPayload
): Promise<SendResult> {
  if (!config.url) return { ok: false, status: null, error: 'URL is empty' };

  try {
    const url = new URL(config.url);
    if (!['http:', 'https:'].includes(url.protocol)) {
      return { ok: false, status: null, error: 'URL must be http:// or https://' };
    }
  } catch {
    return { ok: false, status: null, error: 'Invalid URL format' };
  }

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'User-Agent': 'OmniBridge/0.1',
    ...parseHeaders(config.custom_headers),
  };

  try {
    const res = await fetch(config.url, {
      method: config.method,
      headers,
      body: JSON.stringify(payload),
    });

    let bodyPreview = '';
    try {
      const text = await res.text();
      bodyPreview = text.slice(0, 120);
    } catch {
      // ignore
    }

    return {
      ok: res.ok,
      status: res.status,
      bodyPreview,
      error: res.ok ? undefined : `HTTP ${res.status} ${res.statusText}`,
    };
  } catch (e) {
    return { ok: false, status: null, error: String(e) };
  }
}

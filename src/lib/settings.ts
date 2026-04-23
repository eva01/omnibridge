import { Store } from '@tauri-apps/plugin-store';

const STORE_FILE = 'omnibridge.settings.json';
const KEY_API_KEY = 'anthropic_api_key';

let _store: Store | null = null;

async function getStore(): Promise<Store> {
  if (!_store) {
    _store = await Store.load(STORE_FILE);
  }
  return _store;
}

export async function getApiKey(): Promise<string | null> {
  try {
    const store = await getStore();
    const stored = await store.get<string>(KEY_API_KEY);
    if (stored) return stored;
  } catch {
    // store not available (e.g. browser preview)
  }
  // VITE_-prefixed env vars are inlined into the bundle at build time by Vite.
  // Honoring this fallback in a release build would leak the developer's key
  // into every shipped DMG/MSI, so it is restricted to `npm run tauri dev`.
  if (import.meta.env.DEV) {
    return (import.meta.env.VITE_ANTHROPIC_API_KEY as string) || null;
  }
  return null;
}

export async function setApiKey(key: string): Promise<void> {
  const store = await getStore();
  await store.set(KEY_API_KEY, key);
  await store.save();
}

export async function clearApiKey(): Promise<void> {
  const store = await getStore();
  await store.delete(KEY_API_KEY);
  await store.save();
}

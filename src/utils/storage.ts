import { Store } from '@tauri-apps/plugin-store';
import { HostItem, VpnConfig } from '@/store/hosts';

export interface IStorage {
  theme: ThemeType;
  language: LanguageType;
  hostsList: HostItem[];
  vpnConfig: VpnConfig;
}

async function createStorage<T extends IStorage>() {
  const isTauri = !!window.__TAURI__;

  // Tauri Store implementation
  let store: Store | null = null;
  if (isTauri) {
    store = await Store.load('store.bin');
  }

  // Define methods for Tauri Store
  async function set<K extends keyof T>(key: K, value: T[K]) {
    if (isTauri && store) {
      await store.set(key as string, value);
      await store.save();
    } else {
      localStorage.setItem(key as string, JSON.stringify(value));
    }
  }

  async function get<K extends keyof T>(key: K): Promise<T[K] | null> {
    if (isTauri && store) {
      const value = await store.get<T[K]>(key as string);
      return value !== undefined ? value : null;
    } else {
      const value = localStorage.getItem(key as string);
      return value ? (JSON.parse(value) as T[K]) : null;
    }
  }

  async function remove(key: keyof T) {
    if (isTauri && store) {
      await store.delete(key as string);
    } else {
      localStorage.removeItem(key as string);
    }
  }

  async function clear() {
    if (isTauri && store) {
      await store.clear();
    } else {
      localStorage.clear();
    }
  }

  return {
    set,
    get,
    remove,
    clear,
  };
}

export const storage = await createStorage<IStorage>();

window.$storage = storage;

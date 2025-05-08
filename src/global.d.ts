import type * as TauriApiTypes from '@tauri-apps/api';

declare global {
  interface Window {
    __TAURI__: typeof TauriApiTypes;
    $storage: typeof storage;
  }
}

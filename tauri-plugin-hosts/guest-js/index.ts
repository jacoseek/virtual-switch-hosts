import { invoke } from '@tauri-apps/api/core';

export interface StartVpnRequest {
  address?: string;
  dnsServer?: string;
  routes?: string[];
  httpProxyHost?: string;
  httpProxyPort?: number;
  allowedApplications?: string[];
  disallowedApplications?: string[];
  mtu?: number;
}

type AppInfo = {
  appName: string;
  packageName: string;
  appIcon: string;
};

export async function startVpn(request: StartVpnRequest) {
  return await invoke('plugin:hosts|start_vpn', {
    payload: request,
  });
}

export async function stopVpn() {
  return await invoke('plugin:hosts|stop_vpn');
}

export async function getAppList() {
  return await invoke<{ value: AppInfo[] }>('plugin:hosts|get_app_list');
}

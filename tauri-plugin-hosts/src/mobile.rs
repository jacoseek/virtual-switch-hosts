use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.hosts";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_hosts);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Hosts<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "HostsPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_hosts)?;
    Ok(Hosts(handle))
}

/// Access to the hosts APIs.
pub struct Hosts<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Hosts<R> {
    pub fn start_vpn(&self, payload: StartVpnRequest) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("startVpn", payload)
            .map_err(Into::into)
    }

    pub fn stop_vpn(&self) -> crate::Result<()> {
        self.0.run_mobile_plugin("stopVpn", ()).map_err(Into::into)
    }

    pub fn get_app_list(&self) -> crate::Result<AppListResponse> {
        self.0
            .run_mobile_plugin("getAppList", ())
            .map_err(Into::into)
    }
}

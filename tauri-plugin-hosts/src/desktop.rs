use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Hosts<R>> {
    Ok(Hosts(app.clone()))
}

/// Access to the hosts APIs.
pub struct Hosts<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Hosts<R> {
    pub fn start_vpn(&self, payload: StartVpnRequest) -> crate::Result<()> {
        Ok(())
    }

    pub fn stop_vpn(&self) -> crate::Result<()> {
        Ok(())
    }

    pub fn get_app_list(&self) -> crate::Result<AppListResponse> {
        Ok(AppListResponse::default())
    }
}

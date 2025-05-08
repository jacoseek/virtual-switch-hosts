use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::HostsExt;
use crate::Result;

#[command]
pub(crate) async fn start_vpn<R: Runtime>(
    app: AppHandle<R>,
    payload: StartVpnRequest,
) -> Result<()> {
    app.hosts().start_vpn(payload)
}

#[command]
pub(crate) async fn stop_vpn<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.hosts().stop_vpn()
}

#[command]
pub(crate) async fn get_app_list<R: Runtime>(app: AppHandle<R>) -> Result<AppListResponse> {
    app.hosts().get_app_list()
}

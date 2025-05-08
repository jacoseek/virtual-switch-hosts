use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Hosts;
#[cfg(mobile)]
use mobile::Hosts;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the hosts APIs.
pub trait HostsExt<R: Runtime> {
    fn hosts(&self) -> &Hosts<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HostsExt<R> for T {
    fn hosts(&self) -> &Hosts<R> {
        self.state::<Hosts<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("hosts")
        .invoke_handler(tauri::generate_handler![
            commands::start_vpn,
            commands::stop_vpn,
            commands::get_app_list,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let hosts = mobile::init(app, api)?;
            #[cfg(desktop)]
            let hosts = desktop::init(app, api)?;
            app.manage(hosts);
            Ok(())
        })
        .build()
}

use std::sync::OnceLock;

use log::info;
use tauri_plugin_log::{Target, TargetKind, TimezoneStrategy};
use tauri_specta::{collect_commands, collect_events};

mod tauri_error;
mod virtual_hosts;

use crate::virtual_hosts::*;

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();

macro_rules! specta_builder {
  () => {
    tauri_specta::Builder::<tauri::Wry>::new()
      // Then register them (separated by a comma)
      .commands(collect_commands![
        start_proxy_server,
        stop_proxy_server,
        proxy_server_status,
        set_hosts
      ])
      .events(collect_events![])
  };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_os::init())
    .invoke_handler(specta_builder!().invoke_handler())
    .plugin(tauri_plugin_store::Builder::new().build())
    .plugin(
      // https://aptabase.com/blog/complete-guide-tauri-log
      tauri_plugin_log::Builder::new()
        .targets([
          Target::new(TargetKind::Stdout),
          Target::new(TargetKind::LogDir { file_name: None }),
        ])
        .timezone_strategy(TimezoneStrategy::UseLocal)
        .level(log::LevelFilter::Info)
        .build(),
    )
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_hosts::init())
    .setup(move |app| {
      info!("============== Start App ==============");
      // Global AppHandle
      APP.get_or_init(|| app.handle().clone());

      specta_builder!().mount_events(app);

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[cfg(test)]
mod test {
  use super::*;
  use specta_typescript::{formatter, BigIntExportBehavior, Typescript};

  #[test]
  fn export_types() {
    specta_builder!()
      .export(
        Typescript::default()
          .formatter(formatter::prettier)
          .bigint(BigIntExportBehavior::Number)
          .header("// @ts-nocheck"),
        "../src/utils/bindings.ts",
      )
      .unwrap_or_else(|err: specta_typescript::ExportError| {
        eprintln!("Failed to export TypeScript bindings: {}", err);
        std::process::exit(1);
      });
  }
}

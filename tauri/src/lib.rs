use base64::{prelude::BASE64_STANDARD, Engine};
use image::{ImageFormat, ImageReader};
use std::io::Cursor;
use tauri::{command, AppHandle, Manager, Runtime};

mod gaia_core;

#[command]
fn close_app<R: Runtime>(app: AppHandle<R>) -> Result<(), tauri::Error> {
    app.get_webview_window("main").unwrap().close()
}

#[command]
fn minimize_app<R: Runtime>(app: AppHandle<R>) -> Result<(), tauri::Error> {
    app.get_webview_window("main").unwrap().minimize()
}

#[command]
fn maximize_app<R: Runtime>(app: AppHandle<R>) -> Result<(), tauri::Error> {
    let window = app.get_webview_window("main").unwrap();
    if window.is_maximized().unwrap() {
        window.unmaximize()
    } else {
        window.maximize()
    }
}

#[command]
fn image_from_path_to_base64(path: String) -> Option<String> {
    let img = ImageReader::open(path).ok()?.decode().ok()?;
    let mut buf = Cursor::new(vec![]);
    img.write_to(&mut buf, ImageFormat::Png).ok()?;
    Some(BASE64_STANDARD.encode(buf.get_ref()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                let _ = app.handle().plugin(gaia_core::init());
            }

            Ok(())
        })
        .plugin(gaia_core::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            close_app,
            minimize_app,
            maximize_app,
            image_from_path_to_base64
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

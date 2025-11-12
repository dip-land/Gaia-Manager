#![cfg(all(desktop))]
use fs_more::directory::{
    copy_directory, BrokenSymlinkBehaviour, DestinationDirectoryRule, DirectoryCopyDepthLimit,
    DirectoryCopyOptions, SymlinkBehaviour,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, Window,
};
use toml;
use uuid::Uuid;

mod core_structs;
use crate::gaia_core::core_structs::{Config, ServerStruct};

#[command]
pub fn create_server<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    server: ServerStruct,
) -> Result<String, String> {
    let app_dir = app.path().app_data_dir().unwrap();
    let server = server.to_owned();
    // Check if server path exists
    if fs::exists(server.clone().path).unwrap_or_else(|_| return false) == false {
        fs::create_dir_all(server.clone().path).unwrap_or_else(|err| panic!("{}", err));
        copy_directory(
            path,
            server.clone().path,
            DirectoryCopyOptions {
                destination_directory_rule: DestinationDirectoryRule::AllowEmpty,
                copy_depth_limit: DirectoryCopyDepthLimit::Unlimited,
                symlink_behaviour: SymlinkBehaviour::Keep,
                broken_symlink_behaviour: BrokenSymlinkBehaviour::Keep,
            },
        )
        .unwrap_or_else(|err| panic!("{}", err));
    }
    let jar_path = Path::join(PathBuf::from(server.clone().path).as_path(), "server.jar")
        .to_str()
        .unwrap()
        .to_string();
    let server_config = toml::to_string(&ServerStruct {
        id: Uuid::new_v4(),
        status: 0,
        name: server.clone().name,
        description: server.clone().description,
        port: server.clone().port,
        path: server.clone().path,
        icon_path: Path::join(
            PathBuf::from(server.clone().path).as_path(),
            "server-icon.png",
        )
        .to_str()
        .unwrap()
        .to_string(),
        // TODO: Add some way to detect server type eg. fabric, forge, vanilla, etc...
        server_type: "unknown".to_string(),
        server_jar_path: jar_path.clone(),
        // TODO: Add some way to get minecraft version and server jar version
        minecraft_version: "unknown".to_string(),
        server_version: "unknown".to_string(),
        // TODO: Implement creation_date
        creation_date: "unkown".to_string(),
        auto_start: server.clone().auto_start,
        force_save_on_stop: server.clone().force_save_on_stop,
        // TODO: Add some way to get all of the avaliable java paths
        java_path: "unknown".to_string(),
        java_allocated_memory: server.clone().java_allocated_memory,
        java_startup_line: format!(
            "java -Xmx{}M -Xms256M -jar {} nogui",
            server.clone().java_allocated_memory,
            jar_path.clone()
        ),
        ..Default::default()
    });
    Ok("Success".to_string())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("gaia_core")
        .invoke_handler(tauri::generate_handler![create_server])
        .setup(|app, _api| {
            let app_dir = app.path().app_data_dir().unwrap();
            if fs::exists(app_dir.clone()).unwrap_or_else(|_| return false) == false {
                fs::create_dir(app_dir.clone()).unwrap_or_else(|err| println!("{}", err));
                fs::create_dir(Path::join(app_dir.clone().as_path(), "data"))
                    .unwrap_or_else(|err| println!("{}", err));
            }
            if Path::join(app_dir.clone().as_path(), "config.toml")
                .try_exists()
                .unwrap_or_else(|_| return false)
                == false
            {
                let default_config = toml::to_string(&Config {
                    ..Default::default()
                })
                .unwrap();
                fs::write(
                    Path::join(app_dir.clone().as_path(), "config.toml").as_path(),
                    default_config,
                )
                .unwrap_or_else(|err| println!("{}", err));
            }
            for (_, window) in app.webview_windows() {
                let _ = window.set_title(format!("{:?}", app_dir.clone()).as_str());
            }
            Ok(())
        })
        .build()
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub auto_update: bool,
    pub auto_start: bool,
    pub hide_ip: bool,
    pub startup_display_id: i16,
    pub start_minimized: bool,
    pub start_in_tray: bool,
    pub interface: InterfaceConfig,
}

#[derive(Deserialize, Serialize)]
pub struct InterfaceConfig {
    pub theme: String,
    pub player_heads_enabled: bool,
    pub console: InterfaceConsoleConfig,
}

#[derive(Deserialize, Serialize)]
pub struct InterfaceConsoleConfig {
    pub text_size: String,
    pub font: String,
    pub chat_mode_enabled_default: bool,
    pub auto_scroll_enabled_default: bool,
    pub display_date: bool,
    pub display_year: bool,
    pub timestamp_color: String,
    pub server_started_color: String,
    pub player_joined_color: String,
    pub info_color: String,
    pub warn_color: String,
    pub error_color: String,
    pub background_color: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            auto_update: true,
            auto_start: false,
            hide_ip: false,
            startup_display_id: 0,
            start_minimized: false,
            start_in_tray: false,
            interface: InterfaceConfig {
                ..Default::default()
            },
        }
    }
}

impl Default for InterfaceConfig {
    fn default() -> InterfaceConfig {
        InterfaceConfig {
            theme: "default".to_string(),
            player_heads_enabled: true,
            console: InterfaceConsoleConfig {
                ..Default::default()
            },
        }
    }
}

impl Default for InterfaceConsoleConfig {
    fn default() -> InterfaceConsoleConfig {
        InterfaceConsoleConfig {
            text_size: "16px".to_string(),
            font: "default".to_string(),
            chat_mode_enabled_default: false,
            auto_scroll_enabled_default: true,
            display_date: true,
            display_year: false,
            timestamp_color: "limegreen".to_string(),
            server_started_color: "limegreen".to_string(),
            player_joined_color: "dodgerblue".to_string(),
            info_color: "white".to_string(),
            warn_color: "orange".to_string(),
            error_color: "orangered".to_string(),
            background_color: "rgb(20, 20, 20)".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerStruct {
    pub id: Uuid,
    pub status: i8,
    pub name: String,
    pub description: String,
    pub port: u16,
    pub public_ip: String,
    pub local_ip: String,
    pub path: String,
    pub icon_path: String,
    pub server_type: String,
    pub server_jar_path: String,
    pub minecraft_version: String,
    pub server_version: String,
    pub creation_date: String,
    pub auto_start: bool,
    pub force_save_on_stop: bool,
    pub java_path: String,
    pub java_allocated_memory: i64,
    pub java_startup_line: String,
    pub current_player_count: i32,
    pub max_players: i32,
}

impl Default for ServerStruct {
    fn default() -> ServerStruct {
        ServerStruct {
            id: Uuid::nil(),
            status: 0,
            name: "".to_string(),
            description: "Server Description (This value can be anything)".to_string(),
            port: 25565,
            public_ip: "0.0.0.0".to_string(),
            local_ip: "0.0.0.0".to_string(),
            path: "path".to_string(),
            icon_path: "path".to_string(),
            server_type: "vanilla".to_string(),
            server_jar_path: "path".to_string(),
            minecraft_version: "1.21.10".to_string(),
            server_version: "1.21.10".to_string(),
            creation_date: "today".to_string(),
            auto_start: false,
            force_save_on_stop: true,
            java_path: "path".to_string(),
            java_allocated_memory: 4096,
            java_startup_line: "".to_string(),
            current_player_count: 0,
            max_players: 20,
        }
    }
}

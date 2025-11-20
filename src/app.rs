#![allow(non_snake_case)]
use crate::components;
use dioxus::{document, prelude::*};
use rand::prelude::*;
use uuid::Uuid;
use wasm_bindgen::prelude::*;

pub static APP_STATE: GlobalSignal<AppStateStruct> = Global::new(|| AppStateStruct {
    ..Default::default()
});

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn App() -> Element {
    let server = APP_STATE.read().selected_server.to_owned();
    let _ = use_hook(|| {
        if server.is_some() {
            let value: &mut String = &mut APP_STATE.write().selected_panel;
            *value = format!("SERVER:{}", server.unwrap().id);
        }
    });
    rsx! {
        document::Stylesheet { href: asset!("/assets/styles/main.scss") }
        document::Meta {
            name: "viewport",
            content: "width=device-width, initial-scale=1"
        }
        components::titlebar::title_bar {}
        components::sidebar::side_bar {}
        main {
            class: "container",
            components::server_panel::main_panel {}
            components::create_server_panel::main_panel {}
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AppStateStruct {
    pub selected_panel: String,
    pub selected_sub_panel: String,
    pub selected_server: Option<ServerStruct>,
    pub servers: Vec<ServerStruct>,
    pub server_creation_options: Option<ServerCreationStruct>,
}

impl Default for AppStateStruct {
    fn default() -> Self {
        AppStateStruct {
            selected_panel: "".to_string(),
            selected_sub_panel: "".to_string(),
            selected_server: None,
            servers: Vec::new(),
            server_creation_options: None,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ServerCreationStruct {
    // Values will be "new", "import" or "clone"
    pub creation_type: String,
    pub original_path: Option<String>,
    pub destination_path: Option<String>,
    pub icon_path: Option<String>,
    pub name: String,
    pub description: String,
    pub server_type: String,
    pub port: u16,
    pub java_allocated_memory: i64,
    pub max_players: i32,
    pub auto_accept_eula: bool,
}

impl Default for ServerCreationStruct {
    fn default() -> Self {
        ServerCreationStruct {
            creation_type: "new".to_string(),
            original_path: None,
            destination_path: None,
            icon_path: None,
            name: "server name".to_string(),
            description: "".to_string(),
            server_type: "vanilla".to_string(),
            port: 25565,
            java_allocated_memory: 1024,
            max_players: 20,
            auto_accept_eula: false,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
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
    fn default() -> Self {
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

pub fn create_server_from_creation_struct(creation_struct: ServerCreationStruct) {}

pub fn generate_random_server() -> ServerStruct {
    let mut rng = rand::rng();
    let status: Vec<i8> = (0..2).collect();
    let status = status.choose(&mut rng).unwrap().to_owned();
    let port: Vec<u16> = (0..65535).collect();
    let port = port.choose(&mut rng).unwrap().to_owned();
    let max_players: Vec<i32> = (5..100).collect();
    let max_players = max_players.choose(&mut rng).unwrap().to_owned();
    let players: Vec<i32> = (0..max_players).collect();
    let mut players = players.choose(&mut rng).unwrap().to_owned();
    if status == 0 {
        players = 0;
    }
    let id: Vec<i32> = (0..999999).collect();
    let id = id.choose(&mut rng).unwrap().to_owned();
    let selectedIcon: Vec<i8> = (0..5).collect();
    let selectedIcon = selectedIcon.choose(&mut rng).unwrap().to_owned();
    let mut loader = "vanilla".to_string();
    let mut icon = "E:/MC_Server_Manager/assets/images/dioxus.png".to_string();
    if selectedIcon == 1 {
        icon = "E:/MC_Server_Manager/assets/images/pack.webp".to_string();
    } else if selectedIcon == 2 {
        icon = "E:/MC_Server_Manager/assets/images/fabric.png".to_string();
        loader = "fabric".to_string();
    } else if selectedIcon == 3 {
        icon = "E:/MC_Server_Manager/assets/images/forge.jpg".to_string();
        loader = "forge".to_string();
    } else if selectedIcon == 4 {
        icon = "E:/MC_Server_Manager/assets/images/neoforge.png".to_string();
        loader = "neoforge".to_string();
    }
    let uuid = Uuid::new_v4();
    ServerStruct {
        id: uuid,
        status: status,
        name: format!("Server {}", id),
        port: port,
        icon_path: icon,
        server_type: loader,
        current_player_count: players,
        max_players: max_players,
        ..Default::default()
    }
}

pub fn load_servers() {
    APP_STATE.write().servers.push(generate_random_server());
}

pub fn set_selected_server(value: Uuid) {
    let value = APP_STATE
        .read()
        .servers
        .clone()
        .into_iter()
        .find(|server| server.id == value);
    let app_state = &mut APP_STATE.write();
    app_state.selected_server = Some(value.unwrap());
}

pub fn set_selected_panel(value: &str) {
    reset_option_states();
    let selected_panel: &mut String = &mut APP_STATE.write().selected_panel;
    *selected_panel = value.to_string();
}

pub fn set_selected_sub_panel(value: &str) {
    let selected_sub_panel: &mut String = &mut APP_STATE.write().selected_sub_panel;
    *selected_sub_panel = value.to_string();
}

pub fn set_server_creation_options(value: Option<ServerCreationStruct>) {
    let app_state = &mut APP_STATE.write();
    app_state.server_creation_options = value;
}

pub fn reset_option_states() {
    let app_state = &mut APP_STATE.write();
    app_state.selected_server = None;
    app_state.server_creation_options = None;
}

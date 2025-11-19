use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

use crate::{
    app::{APP_STATE, set_selected_sub_panel},
    components::svgs::{caret_down, play, rotate, skull, stop},
};

#[component]
pub fn main_panel() -> Element {
    let selected = APP_STATE.read().selected_panel.to_string();
    if selected.starts_with("SERVER:") == false {
        return rsx! {};
    }
    rsx! {
        div {
            id: "server_panel",
            server_header {}
            server_chips {}
            sub_panel {}
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ReadFileBytesArgs {
    path: String,
}

fn server_header() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "server_header",
            div {
                id: "server_info",
                server_icon {}
                div {
                    span {
                        class: "server_name",
                        "{server.name}"
                    }
                    span {
                        class: "server_description",
                        "{server.description}"
                    }
                },
            }
            server_actions {}
        }
    }
}

fn server_icon() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    let mut response = use_signal(|| "".to_string());
    // check to prevent infinite image loading loop
    let mut server_id = use_signal(|| Uuid::nil());
    if server_id.read().to_owned() != server.clone().id {
        server_id.set(server.clone().id);
        // fetch bas64 image
        spawn(async move {
            let args = serde_wasm_bindgen::to_value(&ReadFileBytesArgs {
                path: server.icon_path.to_owned(),
            })
            .unwrap();
            let resource_value = invoke("image_from_path_to_base64", args)
                .await
                .as_string()
                .unwrap_or_else(|| "".to_string());
            response.set(resource_value);
        });
    }
    rsx! {
        img {
            class: "server_icon",
            src: format!("data:image/png;base64, {}", response()),
        }
    }
}

fn server_actions() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    let mut menu_opened = use_signal(|| false);
    let menu_opened_value: &bool = &menu_opened.read();
    let menu_opened_value_static = menu_opened_value.clone();
    // if server is offline show start else show full actions
    if server.status == 0 {
        rsx! {
            div {
                class: "server_actions offline",
                div {
                    play::svg {}
                    "Start"
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "server_actions online",
                div {
                    stop::svg {}
                    "Stop Server"
                }
                div {
                    class: if menu_opened_value_static {
                        "rotate"
                    },
                    onclick: move |_| {
                        if menu_opened_value_static {
                            menu_opened.set(false)
                        } else {
                            menu_opened.set(true)
                        }
                    },
                    caret_down::svg {}
                }
            }
            if menu_opened_value_static {
                div {
                    class: "server_actions_menu",
                    div {
                        skull::svg {}
                        "Terminate"
                    }
                    div {
                        rotate::svg {}
                        "Restart"
                    }
                }
            }
        }
    }
}

fn server_chips() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "server_chips",
            if server.status == 0 {
                div {
                    id: "server_status",
                    class: "chip offline",
                    div {
                        class: "status_icon"
                    }
                    "Offline"
                }
            } else {
                div {
                    id: "server_status",
                    class: "chip online",
                    div {
                        class: "status_icon"
                    }
                    "Online"
                }
            }
            div {
                id: "server_ip_chip",
                class: "chip",
                div {
                    class: "icon"
                }
                "{server.local_ip}:{server.port}"
            }
            div {
                id: "player_count_chip",
                class: "chip",
                div {
                    class: "icon"
                }
                "{server.current_player_count}/{server.max_players} Players"
            }
            div {
                id: "server_platform",
                class: "chip",
                div {
                    class: "icon"
                }
                "{server.minecraft_version} | {server.server_type} {server.server_version}"
            }
        }
    }
}

fn sub_panel() -> Element {
    let selected = APP_STATE.read().selected_sub_panel.to_owned();
    rsx! {
    // div {
    //     id: "sub_panel",
        div {
            class: "sub_panel_header",
            div {
                class: if selected == "dashboard".to_string() {
                     "selected"
                },
                onclick: move |_| set_selected_sub_panel("dashboard"),
                "Dashboard"
            }
            div {
                class: if selected == "players".to_string() {
                     "selected"
                },
                onclick: move |_| set_selected_sub_panel("players"),
                "Players"
            }
            div {
                class: if selected == "backups".to_string() {
                     "selected"
                },
                onclick: move |_| set_selected_sub_panel("backups"),
                "Backups"
            }
            div {
                class: if selected == "scheduler".to_string() {
                     "selected"
                },
                onclick: move |_| set_selected_sub_panel("scheduler"),
                "Scheduler"
            }
            // div {
            //     class: if selected == "maps".to_string() {
            //          "selected"
            //     },
            //     onclick: move |_| set_selected_sub_panel("maps"),
            //     "Maps"
            // }
            div {
                class: if selected == "config_editor".to_string() {
                     "selected"
                },
                onclick: move |_| set_selected_sub_panel("config_editor"),
                "Config Editor"
            }
            div {
                class: if selected == "settings".to_string() {
                     "selected"
                },
                onclick: move |_| set_selected_sub_panel("settings"),
                "Settings"
            }
        }
        sub_panel_display {}
    }
    // }
}

fn sub_panel_display() -> Element {
    let selected = APP_STATE.read().selected_sub_panel.to_owned();
    rsx! {
        div {
            id: "sub_panel",
            if selected == "dashboard".to_string() {
                 sub_panel_dashboard {}
            }
            if selected == "players".to_string() {
                 sub_panel_players {}
            }
            if selected == "backups".to_string() {
                 sub_panel_backups {}
            }
            if selected == "scheduler".to_string() {
                 sub_panel_scheduler {}
            }
            if selected == "maps".to_string() {
                 sub_panel_maps {}
            }
            if selected == "config_editor".to_string() {
                 sub_panel_config_editor {}
            }
            if selected == "settings".to_string() {
                 sub_panel_settings {}
            }
        }
    }
}

// DASHBOARD
fn sub_panel_dashboard() -> Element {
    rsx! {
        div {
            id: "sub_panel_dashboard",
            dashboard_console {}
            dashboard_online_players {}
        }
    }
}

fn dashboard_console() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "dashboard_console",
            pre {
                id: "console",
                div {
                    class: "serverstarted",
                    span {
                        class: "timestamp",
                        "[10/26 15:58:02] "
                    }
                    span {
                        class: "logfrom",
                        "[Server thread/INFO] [minecraft/DedicatedServer]: "
                    }
                    span {
                        class: "content",
                        "Done (145.789s)! For help, type \"help\" or \"?\""
                    }
                }
                div {
                    class: "info",
                    span {
                        class: "timestamp",
                        "[10/26 15:58:02] "
                    }
                    span {
                        class: "logfrom",
                        "[Netty Server IO #6/INFO] [FML]: "
                    }
                    span {
                        class: "content",
                        "Client attempting to join with 225 mods"
                    }
                }
                div {
                    class: "userjoined",
                    span {
                        class: "timestamp",
                        "[10/26 15:58:02] "
                    }
                    span {
                        class: "logfrom",
                        "[Server thread/INFO] [minecraft/PlayerList]: "
                    }
                    span {
                        class: "content",
                        "SeaBass7612[/0.0.0.0:60963] logged in with entity id 140 at (632.4353965983014, 6.0, -1708.403939166827)"
                    }
                }
                div {
                    class: "warn",
                    span {
                        class: "timestamp",
                        "[10/26 15:58:02] "
                    }
                    span {
                        class: "logfrom",
                        "[Server thread/WARN] [minecraft/MinecraftServer]: "
                    }
                    span {
                        class: "content",
                        "Can't keep up! Did the system time change, or is the server overloaded? Running 176809ms behind, skipping 3536 tick(s)"
                    }
                }
                div {
                    class: "error",
                    span {
                        class: "timestamp",
                        "[10/26 15:58:02] "
                    }
                    span {
                        class: "logfrom",
                        "[Server thread/INFO] [minecraft/MinecraftServer]: "
                    }
                    span {
                        class: "content",
                        "Stopping server"
                    }
                }
            }
            div {
                input {
                    id: "console_input",
                    type: "text"
                }
                div {
                    label {
                        for: "chat_mode",
                        "Chat Mode"
                    }
                    input {
                        id: "chat_mode",
                        type: "checkbox"
                    }
                }
                div {
                    label {
                        for: "auto_scroll",
                        "Auto Scroll"
                    }
                    input {
                        id: "auto_scroll",
                        type: "checkbox"
                    }
                }
            }
        }
    }
}

fn dashboard_online_players() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "dashboard_players",
            div {
                class: "dashboard_subheader",
                {format!("Online Players {}/{}", server.current_player_count, server.max_players)}
            }
            div {
                class: "player_list",
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
                dashboard_player {}
            }
        }
    }
}

fn dashboard_player() -> Element {
    rsx! {
        div {
            class: "dashboard_player",
            img {
                class: "player_icon",
                src: "r"
            }
            span {
                class: "player_name",
                "USERNAME_THATS_QUITE_LONG"
            }
            button {
                caret_down::svg {}
            }
        }
    }
}

// PLAYERS
fn sub_panel_players() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "sub_panel_players",
            {format!("players display for server {:?}", server)}
        }
    }
}

// BACKUPS
fn sub_panel_backups() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "sub_panel_backups",
            {format!("backups display for server {:?}", server)}
        }
    }
}

// SCHEDUELER
fn sub_panel_scheduler() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "sub_panel_scheduler",
            {format!("scheduler display for server {:?}", server)}
        }
    }
}

// MAPS
fn sub_panel_maps() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "sub_panel_maps",
            {format!("maps display for server {:?}", server)}
        }
    }
}

// CONFIG EDITOR
fn sub_panel_config_editor() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "sub_panel_config_editor",
            {format!("config_editor display for server {:?}", server)}
        }
    }
}

// SETTINGS
fn sub_panel_settings() -> Element {
    let server = APP_STATE.read().selected_server.to_owned().unwrap();
    rsx! {
        div {
            id: "sub_panel_settings",
            {format!("settings display for server {:?}", server)}
        }
    }
}

use std::format;

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
    app::{
        APP_STATE, ServerStruct, load_servers, set_selected_panel, set_selected_server,
        set_selected_sub_panel,
    },
    components::svgs::{folder, gear, plus},
};

#[component]
pub fn side_bar() -> Element {
    rsx! {
        div {
            class: "app_sidebar",
            // div {
            //     class: "server_container",
                server_list {}
                div {
                    class: "extras",
                    div {
                        onclick: move |_| {
                            set_selected_panel("create_server");
                            set_selected_sub_panel("select_server_type");
                        },
                        class: "server_icon",
                        plus::svg {}
                    }
                    div {
                        class: "server_icon",
                        "edit_quick_list"
                    }
                    div {
                        class: "server_icon",
                        folder::svg {}
                    }
                    div {
                        class: "server_icon",
                        gear::svg {}
                    }
                }
            // }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ServerListItemProps {
    id: String,
    name: String,
    icon: String,
    status: String,
    players: Option<String>,
    selected: bool,
}

#[component]
pub fn server_list() -> Element {
    let servers_lock = APP_STATE.read().servers.to_owned();
    let servers_rendered = servers_lock
        .iter()
        .map(|server| server_list_item(server.clone()));

    rsx! {
        div {
            class: "server_list",
            {servers_rendered}
            div {
                class: "debug_info",
                h3 {
                    "DEBUG INFO"
                }
                {format!("Selected Panel: {}", APP_STATE.read().selected_panel)}
                br {}
                {format!("Selected Sub Panel: {}", APP_STATE.read().selected_sub_panel)}
                br {}
                {format!("Selected Server: {:?}", APP_STATE.read().selected_server)}
                br {}
                {format!("Server Creation Options: {:?}", APP_STATE.read().server_creation_options)}
                br {}
                button {
                    onclick: move |_| load_servers(),
                    "Add placeholder server"
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ReadFileBytesArgs {
    path: String,
}

#[component]
pub fn server_list_item(props: ServerStruct) -> Element {
    let players = match props.status == 1 {
        true => format!(
            "{}/{} Players",
            props.current_player_count, props.max_players
        ),
        false => "".to_string(),
    };
    let status = match props.status == 1 {
        true => "online",
        false => "offline",
    };
    let id = props.clone().id;
    let mut base64 = use_signal(|| String::from(""));
    let server_clone = use_signal(|| props.clone());
    let _ = use_resource(move || async move {
        let args = serde_wasm_bindgen::to_value(&ReadFileBytesArgs {
            path: server_clone.read().icon_path.to_owned(),
        })
        .unwrap();
        let resource_value = invoke("image_from_path_to_base64", args)
            .await
            .as_string()
            .unwrap_or_else(|| "".to_string());
        base64.set(resource_value.clone());
        resource_value
    });
    let selected_panel = APP_STATE.read().selected_panel.to_owned();
    rsx! {
        div {
            onclick: move |_| {
                set_selected_panel(format!("SERVER:{}", id).as_str());
                // this will eventually be a changeable user setting
                set_selected_sub_panel("dashboard");
                // server needs to be set last to prevent overrides
                set_selected_server(id);
            },
            id: format!("view_server_{}", props.id),
            class: if selected_panel.starts_with("SERVER:") && Uuid::parse_str(selected_panel.replace("SERVER:", "").as_str()).unwrap() == id {
                "selected"
            },
            img {
                class: "server_icon",
                src: format!("data:image/png;base64, {}", base64.read())
            }
            div {
                class: "server_info",
                span {
                    class: "server_name",
                    "{props.name}"
                }
                div {
                    class: "server_extended_info",
                    span {
                        class: status.to_lowercase(),
                        "{status}"
                    }
                    span {
                        "{players}"
                    }
                }
            }
        }
    }
}

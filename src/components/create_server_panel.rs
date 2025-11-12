use dioxus::prelude::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

use crate::app::{APP_STATE, change_selected_sub_panel};

#[component]
pub fn main_panel() -> Element {
    let selected = APP_STATE.read().selected_panel.to_string();
    let selected_sub_panel = APP_STATE.read().selected_sub_panel.to_string();
    if selected.starts_with("create_server") == false {
        return rsx! {};
    }
    if selected_sub_panel == "select_server_type".to_string() {
        rsx! {
            div {
                id: "create_server_panel",
                div {
                    class: "panel_header",
                    div {
                        h1 {
                            "Create Server"
                        }
                        h3 {
                            "Select what type of server you're going to use."
                        }
                        h3 {
                            style: "margin-bottom: 0.2rem; margin-left: -1rem;",
                            "Playable Servers"
                        }
                    }
                }
                div {
                    class: "sub_section",
                    server_type {
                        name: "Vanilla Java Edition".to_string(),
                        description: "Basic vanilla, no plugins or mods".to_string(),
                        asset: asset!("/assets/images/Grass_Block_JE7_BE6.png"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                    server_type {
                        name: "Paper".to_string(),
                        description: "Based on Spigot, designed to greatly improve performance".to_string(),
                        asset: asset!("/assets/images/papermc_logo.256.webp"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                    server_type {
                        name: "NeoForge".to_string(),
                        description: "Fork of Forge without the controversies, NeoForge does not work with most Forge mods".to_string(),
                        asset: asset!("/assets/images/neoforge.png"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                    server_type {
                        name: "Forge".to_string(),
                        description: "One of the oldest Minecraft Modloaders, Forge does not work with most NeoForge mods".to_string(),
                        asset: asset!("/assets/images/forge.jpg"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                    server_type {
                        name: "Fabric".to_string(),
                        description: "Modular, lightweight mod loader, does not support any forge mods".to_string(),
                        asset: asset!("/assets/images/fabric.png"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                }
                div {
                    class: "panel_header",
                    div {
                        h3 {
                            style: "margin-bottom: 0.2rem; margin-left: -1rem;",
                            "Proxies"
                        }
                    }
                }
                div {
                    class: "sub_section",
                    server_type {
                        name: "BungeeCord".to_string(),
                        description: "".to_string(),
                        asset: asset!("/assets/images/pack.webp"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                    server_type {
                        name: "Velocity".to_string(),
                        description: "Modern, high-performance proxy".to_string(),
                        asset: asset!("/assets/images/velocity_logo_white.webp"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                }
                div {
                    class: "panel_header",
                    div {
                        h3 {
                            style: "margin-bottom: 0.2rem; margin-left: -1rem;",
                            "Experimental"
                        }
                    }
                }
                div {
                    class: "sub_section",
                    server_type {
                        name: "Bedrock Edition".to_string(),
                        description: "Cross-platform version of Minecraft".to_string(),
                        asset: asset!("/assets/images/Bedrock_JE2_BE2.png"),
                        onclick: move |_| {}
                    }
                }
                div {
                    class: "panel_header",
                    div {
                        h3 {
                            style: "margin-bottom: 0.2rem; margin-left: -1rem;",
                            "Legacy"
                        }
                    }
                }
                div {
                    class: "sub_section",
                    server_type {
                        name: "CraftBukkit".to_string(),
                        description: "".to_string(),
                        asset: asset!("/assets/images/craftbukkit.png"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                    server_type {
                        name: "Waterfall".to_string(),
                        description: "".to_string(),
                        asset: asset!("/assets/images/velocity_logo_white.webp"),
                        onclick: move |_| {
                            change_selected_sub_panel("configure_new_server");
                        }
                    }
                }
            }
        }
    } else if selected_sub_panel == "configure_new_server".to_string() {
        rsx! {
            div {
                id: "create_server_panel",
                div {
                    class: "panel_header",
                    div {
                        h1 {
                            "Configure Server"
                        }
                        h3 {
                            "Configure your new TYPE server."
                        }
                    }
                    button {
                        onclick: move |_| {
                            change_selected_sub_panel("select_server_type");
                        },
                        "Back"
                    }
                    button {
                        style: "margin-left: 1rem;",
                        "Create Server"
                    }
                }
                div {
                    class: "sub_section",
                }
            }
        }
    } else {
        rsx! {
            div {
                "OOPS! This should never appear, if it did then there was an error."
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ServerTypeProps {
    onclick: EventHandler<MouseEvent>,
    name: String,
    description: String,
    asset: Asset,
}

pub fn server_type(props: ServerTypeProps) -> Element {
    rsx! {
        div {
            class: "server_type",
            onclick: move |evt| props.onclick.call(evt),
            img {
                src: props.asset
            }
            div {
                span {
                    class: "server_type_name",
                    {props.name}
                }
                span {
                    class: "server_type_description",
                    {props.description}
                }
            }
        }
    }
}

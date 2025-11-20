use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::components::svgs::{chrome_close, chrome_maximize, chrome_minimize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct Empty {}

#[component]
pub fn title_bar() -> Element {
    rsx! {
        div {
            class: "app_titlebar",
            div {
                class: "drag_region",
                "data-tauri-drag-region": ""
            }
            div {
                class: "content",
                div {
                    class: "window_title",
                    "Gaia Manager DEV"
                }
                div {
                    class: "buttons",
                    button {
                        onclick: async move |_| {
                            let args = serde_wasm_bindgen::to_value(&Empty {}).unwrap();
                            invoke("minimize_app", args).await;
                            ()
                        },
                        chrome_minimize::svg {}
                    }
                    button {
                        onclick: async move |_| {
                            let args = serde_wasm_bindgen::to_value(&Empty {}).unwrap();
                            invoke("maximize_app", args).await;
                            ()
                        },
                        chrome_maximize::svg {}
                    }
                    button {
                        onclick: async move |_| {
                            let args = serde_wasm_bindgen::to_value(&Empty {}).unwrap();
                            invoke("close_app", args).await;
                            ()
                        },
                        class: "red",
                        chrome_close::svg {}
                    }
                }
            }
        }
    }
}

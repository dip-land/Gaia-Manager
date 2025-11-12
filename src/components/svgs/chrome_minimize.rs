use dioxus::prelude::*;

pub fn svg() -> Element {
    rsx! {
        svg {
            view_box: "0 0 16 16",
            fill: "currentColor",
            path {
                d: "M14 8v1H3V8h11z"
            }
        }
    }
}

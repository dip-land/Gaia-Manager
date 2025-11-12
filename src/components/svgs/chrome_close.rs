use dioxus::prelude::*;

pub fn svg() -> Element {
    rsx! {
        svg {
            view_box: "0 0 16 16",
            fill: "currentColor",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.116 8l-4.558 4.558.884.884L8 8.884l4.558 4.558.884-.884L8.884 8l4.558-4.558-.884-.884L8 7.116 3.442 2.558l-.884.884L7.116 8z"
            }
        }
    }
}

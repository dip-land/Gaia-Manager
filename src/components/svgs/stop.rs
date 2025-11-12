use dioxus::prelude::*;

pub fn svg() -> Element {
    rsx! {
        svg {
            view_box: "0 0 640 640",
            fill: "currentColor",
            path {
                d: "M160 96L480 96C515.3 96 544 124.7 544 160L544 480C544 515.3 515.3 544 480 544L160 544C124.7 544 96 515.3 96 480L96 160C96 124.7 124.7 96 160 96z"
            }
        }
    }
}

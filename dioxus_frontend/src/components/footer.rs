#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    // Best footer I ever done
    rsx!(
        footer {
            style: "
                width: 100%;
                overflow: hidden;
                position: fixed;
                left: 0;
                bottom: 0;
            ",
            a {
                href: "https://github.com/Kbz-8",
                img {
                    src: "smort-footer.png",
                }
            }
        }
    )
}

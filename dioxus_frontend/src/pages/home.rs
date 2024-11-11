#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components;

#[component]
pub fn Home() -> Element {
    rsx!(
        components::nav::NavBar {}
        h1 {
            style: "
                text-align: center;
            ",
            "Rusty"
        }
        News {}
        components::footer::Footer {}
    )
}

#[component]
fn News() -> Element {
    rsx!(
    )
}

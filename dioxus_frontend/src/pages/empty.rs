#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components;

#[component]
pub fn EmptyPage() -> Element {
    rsx!(
        components::nav::NavBar {}
        h1 {
            style: "
                text-align: center;
                font-family: sans-serif;
                font-size: 60px;
            ",
            "Empty Page"
        }
        components::footer::Footer {}
    )
}

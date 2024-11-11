#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

use pages::home::Home;
use pages::empty::EmptyPage;

mod components;
mod pages;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/empty")]
    EmptyPage {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        components::nav::NavBar {}
        h1 {
            style: "font-family: sans-serif;",
            "Page not found"
        }
        p {
            style: "font-family: sans-serif;",
            "We are terribly sorry, but the page you requested doesn't exist."
        }
        pre {
            style: "font-family: sans-serif;",
            color: "red",
            "log:\nattemped to navigate to: {route:?}"
        }
        components::footer::Footer {}
    }
}

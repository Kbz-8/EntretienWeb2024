#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use reqwest::Client;
use serde_json::Value;
use common::{FormContent, News, NewsType};
use wasm_bindgen_futures::spawn_local;
use std::cell::RefCell;
use std::rc::Rc;

use crate::components;

#[component]
pub fn Home() -> Element {
    rsx!(
        components::nav::NavBar {}
        h1 {
            style: "
                text-align: center;
                font-size: 60px;
                font-family: sans-serif;
            ",
            "Rusty"
        }
        Pedago {}
        News {}
        components::footer::Footer {}
    )
}

async fn FetchNews() -> Result<Vec<Value>, reqwest::Error> {
    let client = Client::new(); // dumb, should keep the client alive
    let res = client
        .get("http://127.0.0.1:8000/news")
        .send()
        .await?;

    let data = res
        .json::<Vec<Value>>()
        .await?;
    Ok(data)
}

#[component]
fn News() -> Element {
    let message = Rc::new(RefCell::new("Loading...".to_string()));

    let message_clone = message.clone();
    spawn_local(async move {
        match FetchNews().await {
            Ok(_msg) => {
                message_clone.borrow_mut().clear();
                message_clone.borrow_mut().push_str(&"test".to_string());
            },
            Err(_) => {
                message_clone.borrow_mut().clear();
                message_clone.borrow_mut().push_str("Failed to load message.");
            },
        }
        info!("{}", message_clone.borrow());
    });

    let current_message = message.borrow().clone();
    rsx!(
        p { "{current_message}" }
    )
}

#[component]
fn Pedago() -> Element {
    rsx!(
        div {
            style: "
                align-items: center;
                justify-content: center;
                text-align: center;
                font-family: sans-serif;
            ",
            a {
                style: "
                    color: white;
                    background-color: #6a99e6;
                    border-radius: 30px;
                    border-color: #d3d2d1;
                    border-style: solid;
                    text-decoration: none;
                    padding: 20px;
                    font-size: 50px;
                ",
                href: "https://outlook.office.com/bookwithme/user/24b95f3e04674b5083af9e018d81e1d4%4042angouleme.fr?anonymous&isanonymous=true",
                "Book a Pedago Slot"
            }
        }
    )
}

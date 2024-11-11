#![allow(non_snake_case)]

use dioxus::prelude::*;
use reqwest::Client;
use common::{FormContent, News, NewsType};

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

async fn FetchNews() -> Result<Vec<News>, reqwest::Error> {
    let client = Client::new(); // dumb, should keep the client alive
    let res = client
        .get("http://127.0.0.1:8000/news")
        .send()
        .await?;

    let data = res
        .json::<Vec<News>>()
        .await?;
    Ok(data)
}

#[component]
fn News() -> Element {
    let data = use_resource(move || FetchNews());
    match &*data.read_unchecked() {
        Some(Ok(list)) => {
            rsx! {
                div {
                    style: "
                        display: grid;
                        grid-template-columns: auto auto;
                        gap: 10px;
                        font-size: 18px;
                        font-family: sans-serif;
                        padding: 10px;
                        width: 50rem;
                        margin: auto;
                        margin-bottom: 100px;
                    ",
                    for new in list {
                        div {
                            style: "
                                color: white;
                                background-color: #00babc;
                                border-radius: 30px;
                                border-color: #d3d2d1;
                                border-style: solid;
                                text-decoration: none;
                                text-align: center;
                                padding: 20px;
                                font-size: 18px;
                                overflow: hidden;
                                text-overflow: ellipsis;
                                max-height: 100px;
                            ",
                            "{new.title}"
                        }
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! {"An error occurred while fetching the news {err}"}
        }
        None => {
            rsx! {"Loading news.."}
        }
    }
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
                margin-bottom: 50px;
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

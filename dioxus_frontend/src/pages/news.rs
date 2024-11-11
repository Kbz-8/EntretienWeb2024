#![allow(non_snake_case)]

use dioxus::prelude::*;
use reqwest::Client;
use common::{News, NewsType};

use crate::components;

// disgusting solution but I don't have time

#[component]
pub fn NewsPage(title: String) -> Element {
    let data = use_resource(move || FetchNews());
    match &*data.read_unchecked() {
        Some(Ok(list)) => {
            let mut news = News {
                title: String::from("No title"),
                content: String::from("No content"),
                r#type: NewsType::Intra42
            };
            for new in list {
                if new.title == title {
                    news = new.clone();
                    break;
                }
            }
            rsx! {
                components::nav::NavBar {}
                h1 {
                    style: "
                        text-align: center;
                        font-size: 60px;
                        font-family: sans-serif;
                    ",
                    "{news.title}"
                }
                p {
                    style: "
                        text-align: center;
                        font-family: sans-serif;
                        font-size: 25px;
                    ",
                    "{news.content}"
                }
                components::footer::Footer {}
            }
        }
        Some(Err(err)) => {
            rsx! {
                components::nav::NavBar {}
                "An error occurred while fetching the news {err}"
                components::footer::Footer {}
            }
        }
        None => {
            rsx! {
                components::nav::NavBar {}
                "Loading news..."
                components::footer::Footer {}
            }
        }
    }
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

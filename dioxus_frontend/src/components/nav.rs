#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx!(
        // Ugly CSS af
        nav {
            style: "
                width: 100%;
                padding: 0.25em 3em 0.5em;
                border-bottom-style: solid;
                border-color: #d3d2d1;
            ",
            div {
                style: "
                    display: flex;
                    flex-wrap: nowrap;
                ",
                a {
                    href: "/",
                    img {
                        src: "logo.jpg",
                        width: "180px",
                    }
                }
                div {
                    style: "
                        align-items: center;
                        justify-content: center;
                    ",
                    ul {
                        style: "
                            align-items: center;
                            display: flex;
                            list-style: none;
                        ",
                        li {
                            style: "
                                margin-left: 10px;
                                margin-right: 10px;
                            ",
                            a {
                                style: "
                                    color: black;
                                    border-color: #d3d2d1;
                                    border-style: solid;
                                    background-color: #f3f2f1;
                                    text-decoration: none;
                                    padding: 10px 20px 10px;
                                ",
                                href: "/", "Home"
                            }
                        }
                        li {
                            style: "
                                margin-left: 10px;
                                margin-right: 10px;
                            ",
                            a {
                                style: "
                                    color: black;
                                    border-color: #d3d2d1;
                                    border-style: solid;
                                    background-color: #f3f2f1;
                                    text-decoration: none;
                                    padding: 10px 20px 10px;
                                ",
                                href: "/empty", "About"
                            }
                        }
                        li {
                            style: "
                                margin-left: 10px;
                                margin-right: 10px;
                            ",
                            a {
                                style: "
                                    color: black;
                                    border-color: #d3d2d1;
                                    border-style: solid;
                                    background-color: #f3f2f1;
                                    text-decoration: none;
                                    padding: 10px 20px 10px;
                                ",
                                href: "/empty", "Contact"
                            }
                        }
                    }
                }
            }
        }
    )
}

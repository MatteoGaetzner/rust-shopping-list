use crate::controllers::{delete_item, get_items, post_item};
use dioxus::prelude::*;
use model::PostShopItem;

pub struct ListChanged;

#[component]
pub fn ShoppingList(list_uuid: Signal<String>, change_signal: Signal<ListChanged>) -> Element {
    let items_request = use_resource(move || async move {
        change_signal.read();
        get_items(list_uuid.read().as_str()).await
    });

    match &*items_request.read_unchecked() {
        Some(Ok(list)) => rsx! {
            div {
                class: "grid place-items-center min-h-500",
                ul {
                    class: "menu bg-base-200 w-200 rounded-box gap-1",
                    for item in list {
                        li {
                            key: "{item.uuid}",
                            ShoppingListItemComponent{
                                display_name: &item.title,
                                posted_by: &item.posted_by,
                                list_uuid,
                                item_uuid: &item.uuid,
                                change_signal: change_signal
                            },
                        }
                    }
                }
            }
        },
        Some(Err(err)) => {
            rsx! {
                p {
                    "Error: {err}"
                }
            }
        }
        None => {
            rsx! {
                p {
                    "Loading items..."
                }
            }
        }
    }
}

#[component]
pub fn ShoppingListItemComponent(
    display_name: String,
    posted_by: String,
    list_uuid: Signal<String>,
    item_uuid: String,
    change_signal: Signal<ListChanged>,
) -> Element {
    rsx! {
        div {
            class: "flex items-center space-x-2",
            p {
                class: "grow text-2xl",
                "{display_name}"
            }
            span {
                "posted by {posted_by}"
            }
            ItemDeleteButton {list_uuid, item_uuid, change_signal}
        }
    }
}

#[component]
pub fn ItemInput(list_uuid: Signal<String>, change_signal: Signal<ListChanged>) -> Element {
    let mut item = use_signal(|| "".to_string());
    let mut author = use_signal(|| "".to_string());

    let onsubmit = move |_| {
        spawn({
            async move {
                let item_name = item.read().to_string();
                let author = author.read().to_string();
                let response = post_item(
                    list_uuid.read().as_str(),
                    PostShopItem {
                        title: item_name,
                        posted_by: author,
                    },
                )
                .await;

                if response.is_ok() {
                    change_signal.write();
                }
            }
        });
    };

    rsx! {
        div {
            class: "w-300 m-4 mt-16 rounded",
            form {
                class: "grid grid-cols-3 gap-2",
                onsubmit: onsubmit,
                div {
                    input {
                        value: "{item}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "next item..",
                        r#type: "text",
                        id: "item_name",
                        name: "item_name",
                        oninput: move |e| item.set(e.data.value().clone())
                    }
                }
                div {
                    input {
                        value: "{author}",
                        class: "input input-bordered input-primary w-full",
                        placeholder: "wanted by..",
                        r#type: "text",
                        id: "author",
                        name: "author",
                        oninput: move |e| author.set(e.data.value().clone())
                    }
                }
                button {
                    class: "btn btn-primary w-full",
                    r#type: "submit",
                    "Commit"
                }
            }
        }
    }
}

#[component]
fn ItemDeleteButton(
    list_uuid: Signal<String>,
    item_uuid: String,
    change_signal: Signal<ListChanged>,
) -> Element {
    let onclick = move |_| {
        spawn({
            let item_uuid = item_uuid.clone();
            async move {
                let response = delete_item(list_uuid.read().as_str(), &item_uuid).await;
                if response.is_ok() {
                    change_signal.write();
                }
            }
        });
    };

    rsx! {
    button {
        onclick: onclick,
        class: "btn btn-circle",
            svg {
                class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                fill: "none",
                path {
                    d: "M6 18L18 6M6 6l12 12"
                }
            }
        }
    }
}

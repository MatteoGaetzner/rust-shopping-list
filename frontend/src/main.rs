use dioxus::prelude::*;

mod components;
mod controllers;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App);
}

#[allow(non_snake_case)]
fn App() -> Element {
    rsx! {
        Router::<Route>{}
    }
}

#[derive(Routable, Clone)]
enum Route {
    #[layout(Layout)]
    #[route("/")]
    LoadOrCreateList {},
    #[route("/list/:list_uuid")]
    Home { list_uuid: String },
    #[route("/profile")]
    Profile {},
}

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-base-300",
            div {
                class: "navbar flex",
                div {
                    Link { class: "p-4", to: Route::LoadOrCreateList{}, "Home" }
                    Link { class: "p-4", to: Route::Profile{}, "Profile" }
                }
            }
            div { class: "container mx-auto max-w-[1024px] p-8",
                Outlet::<Route>{}
            }
        }
    }
}

// #[component]
#[component]
pub fn LoadOrCreateList() -> Element {
    let nav = use_navigator();
    let mut list_uuid = use_signal(|| "".to_string());

    let onloadsubmit = move |_| {
        spawn({
            async move {
                let uuid_value = list_uuid.read().clone();
                if !uuid_value.is_empty() {
                    nav.push(Route::Home {
                        list_uuid: uuid_value,
                    });
                }
            }
        });
    };

    let on_create_list_click = move |_| {
        spawn({
            async move {
                let response = controllers::create_list().await;
                if let Ok(created_list) = response {
                    nav.push(Route::Home {
                        list_uuid: created_list.uuid,
                    });
                }
            }
        });
    };

    rsx! {
        div{
            class: "grid place-content-evently grid-cols-1 md:grid-cols-2 w-full gap-4",
            div {
                class: "card glass min-h-500 flex flex-col content-end gap-4 p-4",
                button{
                    class: "btn btn-primary",
                    onclick: on_create_list_click,
                    "Create new List"
                }
            }
            div { class: "card glass min-h-500",
                form {
                    onsubmit: onloadsubmit,
                    div {
                        class: "flex flex-col gap-4 p-4",
                        input{
                            class:"input input-bordered",
                            r#type:"text",
                            placeholder:"Enter UUID here...",
                            id: "uuid",
                            name: "uuid",
                            oninput: move |e| list_uuid.set(e.data.value())
                        }
                        button{
                            class: "btn btn-primary",
                            r#type: "submit",
                            "Load existing List"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Home(list_uuid: String) -> Element {
    let list_uuid = use_signal(|| list_uuid);
    let change_signal = use_signal(|| components::ListChanged);
    rsx! {
        components::ShoppingList{list_uuid, change_signal}
        components::ItemInput{list_uuid, change_signal}
    }
}

#[allow(non_snake_case)]
pub fn Profile() -> Element {
    rsx! {
        div {
            div {
                class: "flex flex-col gap-4 w-full",
                div {
                    class: "flex gap-4 items-center",
                    div {
                        class: "skeleton w-16 h-16 rounded-full shrink-0"
                    }
                    div {
                        class: "flex flex-col hap-4",
                        div {
                            class: "skeleton h-4 w-20"
                        }
                        div {
                            class: "skeleton h-4 w-28"
                        }
                    }
                }
                div {
                    class: "skeleton h-32 w-full"
                }
                div {
                    class: "skeleton h-32 w-full"
                }
            }
        }
    }
}

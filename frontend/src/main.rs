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
    Home {},
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
                    Link { class: "p-4", to: Route::Home{}, "Home" }
                    Link { class: "p-4", to: Route::Profile{}, "Profile" }
                }
            }
            div { class: "container mx-auto max-w-[1024px] p-8",
                Outlet::<Route>{}
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn Home() -> Element {
    let change_signal = use_signal(|| components::ListChanged);
    rsx! {
        components::ShoppingList{change_signal}
        components::ItemInput{change_signal}
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

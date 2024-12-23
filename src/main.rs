use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    launch(App);
}


pub fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/dist/style.css") }
        div { class: "flex flex-col items-center space-y-4 p-6",
            h3 { class: "text-2xl font-bold text-gray-800",
                "High-Five counter: {count}"
            }
            div { class: "flex space-x-4",
                button { 
                    class: "px-4 py-2 bg-green-500 text-white rounded-lg hover:bg-green-600 transition-colors",
                    onclick: move |_| count += 1,
                    "Up high!"
                }
                button { 
                    class: "px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors",
                    onclick: move |_| count -= 1,
                    "Down low!"
                }
            }
        }
    }
}
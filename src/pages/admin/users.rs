use dioxus::prelude::*;

pub fn users() -> Element {
    rsx! {
        div { class: "bg-white rounded-lg shadow",
            // 表头操作区
            div { class: "p-6 border-b border-gray-200",
                div { class: "flex justify-between items-center",
                    h2 { class: "text-xl font-bold", "用户管理" }
                }
            }
        }
    }
} 
use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn AdminLayout(children: Element) -> Element {
    rsx! {
        div { class: "min-h-screen flex flex-col",
            // 头部栏
            header { class: "w-full bg-blue-500 text-white p-4 shadow",
                h1 { class: "text-xl font-bold", "管理后台" }
            }
            // 主内容区
            div { class: "flex flex-1",
                // 侧边栏
                nav { class: "w-64 bg-white shadow-lg flex-shrink-0",
                    div { class: "p-4",
                        Link { 
                            to: "/admin/dashboard", 
                            class: "block py-2 px-4 hover:bg-gray-100 rounded",
                            "仪表盘" 
                        }
                        Link { 
                            to: "/admin/users", 
                            class: "block py-2 px-4 hover:bg-gray-100 rounded",
                            "用户管理" 
                        }
                    }
                }
                // 内容区
                main { class: "flex-1 p-8 overflow-auto",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

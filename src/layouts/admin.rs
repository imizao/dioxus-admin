use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn AdminLayout(children: Element) -> Element {
    println!("Rendering AdminLayout with children");
    rsx! {
        div { class: "min-h-screen bg-gray-100",
            // 侧边栏
            div { class: "fixed left-0 top-0 w-64 h-full bg-white shadow-lg",
                div { class: "p-4 border-b",
                    h2 { class: "text-xl font-bold", "管理后台" }
                }
                nav { class: "p-4",
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
            // 主内容区
            div { class: "ml-64 p-8",
              Outlet::<Route> {}
            }
        }
    }
}

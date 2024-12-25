use dioxus::prelude::*;

pub fn dashboard() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
            // 统计卡片
            div { class: "bg-white rounded-lg shadow p-6",
                h3 { class: "text-lg font-semibold text-gray-700", "总用户数" }
                p { class: "text-3xl font-bold text-blue-600 mt-2", "1,234" }
            }
            div { class: "bg-white rounded-lg shadow p-6",
                h3 { class: "text-lg font-semibold text-gray-700", "今日活跃" }
                p { class: "text-3xl font-bold text-green-600 mt-2", "256" }
            }
            div { class: "bg-white rounded-lg shadow p-6",
                h3 { class: "text-lg font-semibold text-gray-700", "新增用户" }
                p { class: "text-3xl font-bold text-orange-600 mt-2", "32" }
            }
            
            // 图表区域
            div { class: "col-span-full bg-white rounded-lg shadow p-6 mt-6",
                h2 { class: "text-xl font-bold mb-4", "数据趋势" }
                div { class: "h-64 bg-gray-100 rounded",
                    // TODO: 添加图表
                    p { class: "text-center py-24 text-gray-500", "图表区域" }
                }
            }
        }
    }
} 
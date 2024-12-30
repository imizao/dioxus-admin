use dioxus::prelude::*;

mod layouts;
mod pages;

use layouts::admin::AdminLayout;
use pages::admin::dashboard::dashboard as Dashboard;
use pages::admin::users::users as Users;

#[derive(Routable, Clone)]
enum Route {

    #[route("/")]
    Home{},

    #[layout(AdminLayout)]
    #[route("/admin/dashboard")]
    Dashboard,

    #[route("/admin/users")]
    Users,

}

fn main() {
    launch(app);
}

fn app() -> Element {
    println!("App is rendering");
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/dist/style.css") }
        Router::<Route> {}
    }
}

fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center min-h-screen bg-gray-100",
            h1 { class: "text-4xl font-bold mb-8", "管理系统" }
            div { class: "space-x-4",
                Link { 
                    to: "/admin/dashboard",
                    class: "px-6 py-2 bg-blue-500 text-blue-500 rounded-lg hover:bg-blue-600",
                    "进入管理后台" 
                }
            }
        }
    }
}
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn BaseLayout() -> Element {
    rsx! {
        main { class: "scroll-smooth font-sans antialiased bg-background", Outlet::<Route> {} }
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }
    }
}

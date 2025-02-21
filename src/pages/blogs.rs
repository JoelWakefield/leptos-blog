use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn Blogs() -> impl IntoView {
    view! {
      <div>
        <h2>"Blogs"</h2>

        <Outlet />
      </div>
    }
}

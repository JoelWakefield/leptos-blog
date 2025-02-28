use leptos::prelude::*;
use leptos_router::components::Outlet;

use crate::app::services::blog_service::get_blog_meta_list;

#[component]
pub fn Blogs() -> impl IntoView {
  let blogs = get_blog_meta_list();
  let (metas, _) = signal(blogs);

    view! {
      <div>
        <h2>"Blogs"</h2>

        <For 
          each=move || metas.get()
          key=|meta| meta.slug.clone()
          let(child)
        >
          <a href=format!("/blogs/{:?}", child.slug)>
            {child.title}
          </a>
        </For>
      
        <Outlet />
      </div>
    }
}

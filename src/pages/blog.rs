use leptos::{leptos_dom::logging::console_log, prelude::*};
use leptos_router::{hooks::use_params, params::Params};

use crate::app::services::blog_service::get_blog;

#[derive(Params, PartialEq)]
struct BlogParams {
    slug: Option<String>,
}

#[component]
pub fn Blog() -> impl IntoView {
    let params = use_params::<BlogParams>();
    let slug = params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.slug.clone())
            .unwrap_or_default();

    console_log(slug.as_str());
    let blog = get_blog(slug).unwrap();
    console_log(&blog.meta.title);
    let (paragraphs, _) = signal(blog.paragraphs);

    view! {
        <header>
            <h3>{blog.meta.title}</h3>
        </header>
        <article>
            <For 
                each=move || paragraphs.get()
                key=|paragraph| paragraph.section.clone()
                let(child)
            >
                <p>{child.text}</p>
            </For>
        </article>
    }
}

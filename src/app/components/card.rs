use leptos::prelude::*;
use leptos_router::components::{ToHref, A};

#[component]
pub fn Card<H>(href: H, title: &'static str, text: &'static str) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
      <div class="flex-1 min-w-md">
        <A href=href>
          <div class="p-10 rounded-lg bg-(--foreground)">
            <p class="text-black text-4xl">{title}</p>
            <p class="text-(--background) text-lg">{text}</p>
          </div>
        </A>
      </div>
    }
}

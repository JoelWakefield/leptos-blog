use leptos::prelude::*;
use leptos_router::components::{ToHref, A};

#[component]
pub fn NavLink<H>(href: H, label: &'static str) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
      <A href={href}>
        <div class="bg-(--foreground) p-2">
          <p class="text-black text-center text-lg">{label}</p>
        </div>
      </A>
    }
}

use leptos::prelude::*;
use leptos_router::components::{ToHref, A};

#[component]
pub fn Card<H>(
    href: H,
    title: &'static str,
    text: &'static str,
    image_src: &'static str,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
      <div class="w-full py-2 px-14 text-left">
        <A href=href>
          <div class="p-10 flex flex-row rounded-lg bg-(--yellow)">
            <div>
              <h3 class="text-black text-3xl">{title}</h3>
              <p class="text-black text-lg">{text}</p>
            </div>
            <img src={image_src} alt={title} width="240" height="180" />
          </div>
        </A>
      </div>
    }
}

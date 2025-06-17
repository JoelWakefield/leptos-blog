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
      <div class="w-full py-4 px-14 max-w-lg text-left bg-(--lightblue)">
        <A href=href>
          <div class="p-10 flex flex-row rounded-lg bg-(--yellow)">
            <div>
              <p class="text-black text-4xl">{title}</p>
              <p class="text-black text-lg">{text}</p>
            </div>
            <img src={image_src} alt={title} width="128" height="128" />
          </div>
        </A>
      </div>
    }
}

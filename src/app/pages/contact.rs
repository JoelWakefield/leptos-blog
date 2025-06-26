use leptos::prelude::*;

use crate::app::components::contact::ContactCard;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
      <div class="my-12 mx-8 max-w-4xl flex flex-col gap-y-8 items-center text-center">
        <h1 class="text-8xl">"Reach Out"</h1>

        <ContactCard />
      </div>
    }
}

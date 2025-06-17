use leptos::prelude::*;

use crate::app::components::contact::ContactCard;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
      <div class="my-4 mx-auto max-w-lg flex flex-col gap-y-8 items-center text-center">
        <h2 class="text-2xl">"Get In Touch"</h2>

        <ContactCard />
      </div>
    }
}

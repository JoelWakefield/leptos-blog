#[path = "../components/card.rs"]
mod card;

use card::Card;
use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div>
        <div class="m-12 text-center">
          <h1 class="text-4xl">The Try Angle</h1>
          <p>The angle is to try new things and learn as much as possible.</p>
        </div>

        <div class="m-8 flex justify-center gap-4">
          <Card href="/blogs" title="Blogs" text="Page for Blogs" />
          <Card href="/videos" title="Videos" text="Page for Videos" />
          <Card href="/source-codes" title="Source Codes" text="Page for Source Codes" />
        </div>
      </div>
    }
}

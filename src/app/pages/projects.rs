use leptos::prelude::*;

use crate::app::components::card::Card;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
      <div class="m-4 text-center">
        <h2 class="text-2xl">"Projects"</h2>

        <Card
          title="Sailing Sim"
          href="https://github.com/Wake1st/SailingSim"
          text="A sailing simulation slowly taking shape using the godot game engine."
        />

        <Card
          title="Hill Builder"
          href="https://github.com/Wake1st/hill-builder"
          text="A simple game about modifying terrain."
        />

        <Card
          title="Project Management App"
          href=""
          text="An ongoing project to display the differences between Blazor and React."
        />
      </div>
    }
}

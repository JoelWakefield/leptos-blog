use leptos::prelude::*;

use crate::app::components::card::Card;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
      <div class="my-4 mx-auto max-w-lg flex flex-col gap-y-8 items-center text-center">
        <h1 class="text-2xl gap-y-6">"Projects"</h1>

        <div class="flex flex-col ">
          <h2>"Completed"</h2>

          <Card
            title="Gomi Moni"
            href="gomi-moni"
            text="A godot game made for the Pompous Trash Jam 2025."
            image_src="gomi-moni.png"
          />

          <Card
            title="Break Glass"
            href="break-glass"
            text="A 2D obstacle course game made with Godot."
            image_src="break-glass.png"
          />

          <Card
            title="Hot Panda with Moonbeam"
            href="hot-panda"
            text="A 3D fighting game made for the Hexcode Palette Jam."
            image_src="hot-panda.png"
          />

          <Card
            title="Make Words"
            href="make-words"
            text="An interactive experience about putting letters together."
            image_src="make-words.png"
          />
        </div>
      </div>
    }
}

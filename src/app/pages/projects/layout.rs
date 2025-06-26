use leptos::prelude::*;

use crate::app::components::card::Card;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
      <div class="m-auto max-w-4xl flex flex-col items-center text-center">
        <h1 class="my-12 mx-8 text-8xl">"Projects"</h1>

        <Card
          title="Gomi Moni"
          href="gomi-moni"
          text="A godot game made for the Pompous Trash Jam 2025."
          image_src="/public/gomi_moni.png"
        />

        <Card
          title="Break Glass"
          href="break-glass"
          text="A 2D obstacle course game made with Godot."
          image_src="/public/break_glass.png"
        />

        <Card
          title="Hot Panda with Moonbeam"
          href="hot-panda"
          text="A 3D fighting game made for the Hexcode Palette Jam."
          image_src="/public/hot_panda.png"
        />

        <Card
          title="Make Words"
          href="make-words"
          text="An interactive experience about putting letters together."
          image_src="/public/make_words.png"
        />
      </div>
    }
}

use leptos::prelude::*;

use crate::app::components::card::Card;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
      <div class="m-4 text-center">
        <h1 class="text-2xl">"Projects"</h1>

        <Card
          title="Gomi Moni"
          href="gomi-moni"
          text="A godot game made for the Pompous Trash Jam 2025."
        />

        <Card
          title="Break Glass"
          href="break-glass"
          text="A 2d obstacle course game made with Godot."
        />

        <Card
          title="Course Constructor"
          href="course-constructor"
          text="Build race tracks, then race on them."
        />

        <Card
          title="Vox Box"
          href="vox-box"
          text="Record and edit sounds."
        />

        <Card
          title="Hill Builder"
          href="hill-builder"
          text="A simple simulation about modifying terrain."
        />
      </div>
    }
}

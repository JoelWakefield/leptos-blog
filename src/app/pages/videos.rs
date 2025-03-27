use leptos::prelude::*;

use crate::app::components::card::Card;

#[component]
pub fn Videos() -> impl IntoView {
    view! {
      <div class="m-4 text-center">
        <h2 class="text-2xl">"Videos"</h2>

        <Card
          title="Gomi Moni"
          href="https://www.youtube.com/watch?v=GqXTHFAd6DU"
          text="A godot game made for the Pompous Trash Jam 2025."
        />

        <Card
          title="Project Management App"
          href="https://www.youtube.com/watch?v=R-hh4OBnT6A&list=PL5zSWpNQadKNlrrqlNf-Ni5pyGOLFlmVg"
          text="Videos covering my attempt to build a project management app."
        />

        <Card
          title="Sailing Simulation"
          href="https://www.youtube.com/watch?v=ARbbpqTq-kE&list=PL5zSWpNQadKO3uMEvwXZJC3sKMIlIF_bR"
          text="Yo ho, yo ho - I build a sailing game with godot."
        />

        <Card
          title="Bevy Hill Builder"
          href="https://www.youtube.com/watch?v=eU1ZcSliolE&list=PL5zSWpNQadKO3O8Ker6f9HZzaSHGtoRMc"
          text="Using bevy to move water by adjusting terrain."
        />
      </div>
    }
}

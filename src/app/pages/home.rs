use leptos::prelude::*;

use crate::app::components::{card::Card, contact::ContactCard};

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="m-auto max-w-4xl">
        <div class="my-12 mx-8 text-center">
          <h1 class="text-8xl">"The Try Angle"</h1>
          <p class="p-4 text-2xl text-justify">
            "The angle is to try new things and learn as much as possible."
          </p>
        </div>

        <div class="m-12 flex flex-row gap-x-4 justify-between items-stretch">
          <p class="p-4 text-lg text-left">
            "Hello world! Here you shall find a number of projects I've worked on over the past year. While most of my professional career revolves around business applications, in my spare time, I enjoy learning other languages, frameworks, design patterns, etc. This site highlights a few of those projects and documents the process of making them."
          </p>
          <ContactCard />
        </div>

        <div class="m-12 flex flex-col items-center rounded-xl">
          <h2 class="m-4 text-5xl">Highlight Project</h2>

          <Card
            href="/projects/gomi-moni"
            title="Gomi Moni"
            text="A 3D physics puzzle game about trash."
            image_src="/public/gomi_moni.png"
          />
        </div>
      </div>
    }
}

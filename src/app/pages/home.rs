use leptos::prelude::*;

use crate::app::components::card::Card;

#[component]
pub fn Home() -> impl IntoView {
    view! {
      <div class="m-auto max-w-5xl">
        <div class="m-12 text-center">
          <h1 class="m-6 text-4xl">The Try Angle</h1>
          <p text="p-4 text-lg">The angle is to try new things and learn as much as possible.</p>
        </div>

        <div class="m-8 flex flex-col items-center">
          <h2 class="m-4 text-2xl">Internal Links</h2>

          <div class="p-2 flex flex-wrap gap-4">
            <Card href="/projects" title="Projects" text="Page for Projects" />
            <Card href="/videos" title="Videos" text="Page for Videos" />
          </div>
        </div>

        <div class="m-8 flex flex-col items-center">
          <h2 class="m-4 text-2xl">External Links</h2>

          <div class="p-2 flex flex-wrap gap-4">
            <Card
              href="https://github.com/JoelWakefield"
              title="Profesional Github"
              text="Profile for more career focused source code."
            />
            <Card
              href="https://github.com/Wake1st"
              title="Fun Github"
              text="Profile for game dev related projects."
            />
            <Card
              href="https://www.twitch.tv/thetryangle"
              title="Twitch Page"
              text="Stay awhile and watch me try new things."
            />
            <Card
              href="https://www.youtube.com/@the-try-angle"
              title="YouTube Page"
              text="Here I upload my VODs from Twitch."
            />
          </div>
        </div>
      </div>
    }
}

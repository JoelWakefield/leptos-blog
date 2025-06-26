use leptos::prelude::*;

#[component]
pub fn ContactCard() -> impl IntoView {
    view! {
      <div class="w-full py-4 px-14 max-w-lg text-center bg-(--lightblue)">
        <h3 class="pb-6 text-3xl">"Contact Info"</h3>

        <ul class="text-left leading-8 font-extrabold text-(--red)">
          <li><a href="mailto:joel.tryangle@gmail.com">joel.tryangle@gmail.com</a></li>
          <li><a href="https://www.twitch.tv/thetryangle">twitch/thetryangle</a></li>
          <li><a href="https://www.youtube.com/@the-try-angle">youtube/@the-try-angle</a></li>
          <li><a href="https://discord.gg/Pq35pKKT">discord invite</a></li>
        </ul>
      </div>
    }
}

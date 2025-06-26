use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
      <div class="m-auto max-w-4xl">
        <div class="my-12 mx-8 text-center">
          <h1 class="text-8xl gap-y-6">"About Me"</h1>
          <p class="p-4 text-lg text-left">
            "I've been a software developer/engineer for 7+ years, mostly working on business applications in my work, and game dev in my spare time. This channel is about trying new things, specifically game-dev related. Everyday is a day to try something new."
          </p>
        </div>
      </div>
    }
}

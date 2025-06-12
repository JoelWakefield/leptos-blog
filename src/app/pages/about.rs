use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn About() -> impl IntoView {
    view! {
      <div>
        <div>
          <h1>"About Me"</h1>
          <p>"Hello world!"</p>
          <p>
            "I've been a software developer/engineer for 7+ years, mostly working on business applications in my work, and game dev in my spare time. This channel is about trying new things, specifically game-dev related. Everyday is a day to try something new."
          </p>
          <p>
            "This site servers as a portfolio/devlog of my projects. Check out "
            <A href="https://www.twitch.tv/thetryangle">"twitch"</A>
            ", "
            <A href="https://www.youtube.com/@the-try-angle">"youtube"</A>
            ", or "
            <A href="https://discord.gg/Pq35pKKT">"discord"</A>
            " for ways to connect."
          </p>
        </div>
      </div>
    }
}

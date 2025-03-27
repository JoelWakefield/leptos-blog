mod components;
mod pages;

use components::nav_link::NavLink;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use pages::{home::Home, projects::Projects, videos::Videos};

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          <div class="m-auto flex items-centered gap ">
            <NavLink href="/" label="Home" />
            <NavLink href="/projects" label="Projects" />
            <NavLink href="/videos" label="Videos" />
          </div>
        </nav>
        <main>
          <Routes fallback=|| "404: Not Found.">
            <Route path=path!("/") view=Home />
            <Route path=path!("/projects") view=Projects />
            <Route path=path!("/videos") view=Videos />
            <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
          </Routes>
        </main>
      </Router>
    }
}

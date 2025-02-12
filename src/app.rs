#[path = "./components/mod.rs"]
mod components;
#[path = "./pages/mod.rs"]
mod pages;

use components::nav_link::NavLink;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use pages::blogs::Blogs;
use pages::home::Home;
use pages::source_codes::SourceCodes;
use pages::videos::Videos;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          <div class="m-auto flex items-centered gap ">
            <NavLink href="/" label="Home" />
            <NavLink href="/blogs" label="Blogs" />
            <NavLink href="/source-codes" label="Source Codes" />
            <NavLink href="/videos" label="Videos" />
          </div>
        </nav>
        <main>
          <Routes fallback=|| "404: Not Found.">
            <Route path=path!("/") view=Home />
            <Route path=path!("/blogs") view=Blogs />
            <Route path=path!("/source-codes") view=SourceCodes />
            <Route path=path!("/videos") view=Videos />
            <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
          </Routes>
        </main>
      </Router>
    }
}

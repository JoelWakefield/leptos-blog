mod components;
mod pages;

use components::nav_link::NavLink;
use leptos::prelude::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use pages::{
    about::About,
    contact::ContactPage,
    home::Home,
    projects::{
        break_glass::BreakGlass, course_constructor::CourseConstructor,
        game_design_patterns::GameDesignPatterns, gomi_moni::GomiMoni, layout::Projects,
        make_words::MakeWords,
    },
};

use crate::app::pages::projects::hot_panda::HotPanda;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          <div class="m-auto flex items-centered gap bg-(--yellow)">
            <NavLink href="/" label="Home" />
            <NavLink href="/about" label="About Me" />
            <NavLink href="/projects" label="Projects" />
            <NavLink href="/contact" label="ContactPage" />
          </div>
        </nav>
        <main>
          <Routes fallback=|| "404 - Turn Back">
            <Route path=path!("/") view=Home />
            <Route path=path!("/about") view=About />
            <Route path=path!("/contact") view=ContactPage />
            <ParentRoute path=path!("/projects") view=Projects>
              <Route path=path!("/break-glass") view=BreakGlass/>
              <Route path=path!("/course-constructor") view=CourseConstructor/>
              <Route path=path!("/game-design-patterns") view=GameDesignPatterns/>
              <Route path=path!("/gomi-moni") view=GomiMoni/>
              <Route path=path!("/hot-panda") view=HotPanda/>
              <Route path=path!("/make-words") view=MakeWords/>
              <Route path=path!("") view=Projects/>
            </ParentRoute>
            <Route path=path!("/*any") view=|| view! { <h1>"404 - Ain't Nuthin' Here"</h1> }/>
          </Routes>
        </main>
      </Router>
    }
}

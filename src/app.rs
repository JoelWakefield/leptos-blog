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
    home::Home,
    projects::{
        break_glass::BreakGlass, course_constructor::CourseConstructor, gomi_moni::GomiMoni,
        hill_builder::HillBuilder, layout::Projects, vox_box::VoxBox,
    },
};

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          <div class="m-auto flex items-centered gap ">
            <NavLink href="/" label="Home" />
            <NavLink href="/about" label="About Me" />
            <NavLink href="/projects" label="Projects" />
          </div>
        </nav>
        <main>
          <Routes fallback=|| "404 - Turn Back">
            <Route path=path!("/") view=Home />
            <Route path=path!("/about") view=About />
            <ParentRoute path=path!("/projects") view=Projects>
              <Route path=path!(":gomi-moni") view=GomiMoni/>
              <Route path=path!(":break-glass") view=BreakGlass/>
              <Route path=path!(":vox-box") view=VoxBox/>
              <Route path=path!(":course-constructor") view=CourseConstructor/>
              <Route path=path!(":hill-builder") view=HillBuilder/>
              <Route path=path!("") view=Projects/>
            </ParentRoute>
            <Route path=path!("/*any") view=|| view! { <h1>"404 - Ain't Nuthin' Here"</h1> }/>
          </Routes>
        </main>
      </Router>
    }
}

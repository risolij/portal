use yew::prelude::*;

use crate::components::project_list::ProjectList;
use crate::components::nav::Nav;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    html! {
        <main class={classes!("projects_page")}>
            <Nav />
            <ProjectList />
        </main>

    }
}

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::home_page::HomePage;
use crate::pages::projects_page::ProjectsPage;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[at("/projects")]
    ShowProjects,

    #[at("/project/:id")]
    SingleProject { id: usize },

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::ShowProjects => html! { <ProjectsPage /> },
        Route::SingleProject { id } => html! { <h1>{format!("you chose {}", id)}</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

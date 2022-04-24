use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

mod components;
mod models;
mod pages;
mod routes;

use models::project::Project;
use models::state::{Action, State};
use routes::routes::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    let (state, dispatch) = use_store::<State>();

    use_effect(move || {
        if state.first_load.to_owned() {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_projects = reqwest::get(state.api)
                    .await
                    .unwrap()
                    .json::<Vec<Project>>()
                    .await
                    .unwrap();

                dispatch.apply(Action::FirstLoad {
                    projects: fetched_projects,
                    load: false,
                });
            });
        }
        || {}
    });

    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

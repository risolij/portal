use super::project_item::ProjectItem;
use crate::State;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(ProjectList)]
pub fn project_list() -> Html {
    let (state, _) = use_store::<State>();
    let projects = state
        .projects
        .iter()
        .map(|project| html! { <ProjectItem project={project.clone()} /> })
        .collect::<Html>();

    html! {
        <section class={classes!("projects_content")}>
            {projects}
        </section>
    }
}

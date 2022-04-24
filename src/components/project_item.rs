use crate::models::project::Project;
use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub project: Project,
}

#[function_component(ProjectItem)]
pub fn project_item(props: &Props) -> Html {
    html! {
        <section class={classes!("project-card")}>
            <article>
                <ul>
                    <li>{"Full Name: "} {&props.project.full_name}</li>
                    <li>{"Language: "} {&props.project.language}</li>
                    <li>{"Forks: "} {props.project.forks}</li>
                    <li>{"Branch: "} {&props.project.default_branch}</li>
                </ul>
            </article>
            <header>
                <h1><a href={props.project.html_url.clone()}>{&props.project.name}</a></h1>
            </header>
        </section>
    }
}

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::routes::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav>
            <section class={classes!("nav-left")}></section>
            <section class={classes!("nav-middle")}>
                <ul>
                    <li><Link<Route> classes={classes!("active")} to={Route::Home}>{ "home" }</Link<Route>></li>
                    <li><Link<Route> to={Route::ShowProjects}>{ "proj" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Home}>{ "term" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Home}>{ "next" }</Link<Route>></li>
                </ul>
            </section>
            <section class={classes!("nav-right")}></section>
        </nav>
    }
}


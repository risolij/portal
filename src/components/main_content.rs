use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <section class={classes!("content")}>
            <article class={classes!("description")}>
                <h3>{"Hello"}</h3>
                <h1>{"I'm "}<span>{"Jim"}</span>{" Risoli"}</h1>
                <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}</p>
                <button>{"learn"}</button>
            </article>
            <article class={classes!("bg")}></article>
        </section>
    }
}

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub button_name: String,

}

#[function_component(Feedback)]
pub fn feedback(props: &Props) -> Html {
    html! {
        <form>
            <input type="text" />
            <textarea />
            <button>{&props.button_name}</button>
        </form>
    }
}

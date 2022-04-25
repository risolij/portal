use yew::prelude::*;

use crate::components::project_list::ProjectList;
use crate::components::nav::Nav;
use crate::components::main_content::MainContent;
use crate::components::footer::Footer;


#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <main class={"home_page"}>
            <Nav />
            <MainContent />
            <Footer />
        </main>

    }
}

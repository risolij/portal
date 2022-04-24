use gloo::console::log;
use yewdux::prelude::*;

use super::project::Project;

#[derive(Clone, PartialEq, Store)]
pub struct State {
    pub projects: Vec<Project>,
    pub first_load: bool,
    pub api: &'static str,
}

impl Default for State {
    fn default() -> Self {
        Self {
            projects: vec![],
            first_load: true,
            api: "https://api.github.com/users/risolij/repos",
        }
    }
}

pub enum Action {
    FirstLoad { projects: Vec<Project>, load: bool },
}

impl Reducer<State> for Action {
    fn apply(&self, state: &mut State) {
        match self {
            Action::FirstLoad { projects, load } => {
                log!("We hit first load!");
                state.projects = projects.to_owned();
                state.first_load = load.to_owned();
            }
        }
    }
}

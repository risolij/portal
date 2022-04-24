use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Project {
    pub id: usize,
    pub name: String,
    pub full_name: String,
    pub language: String,
    pub default_branch: String,
    pub forks: usize,
    pub html_url: String,
}

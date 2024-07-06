use serde::Deserialize;

pub trait GitHub {
    async fn get_open_issues(&self) -> Result<Vec<Issue>, Box<dyn std::error::Error>>;
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Issue {
    pub number: u32,
    pub title: String,
    pub body: Option<String>,
    pub url: String,
    pub user: User,
}

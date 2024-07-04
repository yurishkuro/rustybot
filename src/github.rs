use serde::Deserialize;

#[derive(Debug)]
pub enum GitHubError {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    Other(u16, String),
}

impl std::fmt::Display for GitHubError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GitHubError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            GitHubError::Json(err) => write!(f, "JSON error: {}", err),
            GitHubError::Other(code, err) => write!(f, "Status: {} - {}", code, err),
        }
    }
}

impl std::error::Error for GitHubError {}

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

pub fn get_open_issues(
    repo_owner: &str,
    repo_name: &str,
    token: &str,
) -> Result<Vec<Issue>, GitHubError> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/issues?state=open",
        repo_owner, repo_name
    );
    let client = reqwest::blocking::Client::new();
    let mut req = client.get(url).header("User-Agent", "rust/reqwest");
    if !token.is_empty() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let res = req.send().map_err(GitHubError::Reqwest)?;
    let status = res.status().as_u16();
    let body = res.text().map_err(GitHubError::Reqwest)?;
    // println!("Status: {}", status);
    // println!("Response body: {}", body);
    if status != 200 {
        return Err(GitHubError::Other(status, body));
    }

    let response: Vec<Issue> = serde_json::from_str(&body).map_err(GitHubError::Json)?;
    Ok(response)
}

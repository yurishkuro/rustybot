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

pub struct Client {
    pub api_url: String,
    pub token: String,
    pub repo_owner: String,
    pub repo_name: String,
}

impl Client {
    pub async fn get_open_issues(&self) -> Result<Vec<Issue>, GitHubError> {
        let url = format!(
            "{}/repos/{}/{}/issues?state=open",
            self.api_url, self.repo_owner, self.repo_name,
        );
        let client = reqwest::Client::new();
        let mut req = client.get(url).header("User-Agent", "rust/reqwest");
        if !self.token.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", self.token));
        }
        let res = req.send().await.map_err(GitHubError::Reqwest)?;
        let status = res.status().as_u16();
        let body = res.text().await.map_err(GitHubError::Reqwest)?;
        // println!("Status: {}", status);
        // println!("Response body: {}", body);
        if status != 200 {
            return Err(GitHubError::Other(status, body));
        }

        let response: Vec<Issue> = serde_json::from_str(&body).map_err(GitHubError::Json)?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_open_issues() {
        let mock_server = MockServer::start().await;
        let body = r#"
            [
                {
                    "number": 1,
                    "title": "Issue 1",
                    "body": "This is issue 1",
                    "url": "https://api.github.com/repos/yurishkuro/rustybot/issues/1",
                    "user": {
                        "login": "yurishkuro"
                    }
                },
                {
                    "number": 2,
                    "title": "Issue 2",
                    "body": "This is issue 2",
                    "url": "https://api.github.com/repos/yurishkuro/rustybot/issues/2",
                    "user": {
                        "login": "yurishkuro"
                    }
                }
            ]
        "#;
        let response = ResponseTemplate::new(200).set_body_string(body);
        Mock::given(method("GET"))
            .and(path("/repos/yurishkuro/rustybot/issues"))
            .and(query_param("state", "open"))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let gh_client = Client {
            api_url: mock_server.uri(),
            token: String::from(""),
            repo_owner: String::from("yurishkuro"),
            repo_name: String::from("rustybot"),
        };
        let result = gh_client.get_open_issues().await;
        assert!(result.is_ok());
        let issues = result.unwrap();
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].number, 1);
        assert_eq!(issues[0].title, "Issue 1");
        assert_eq!(issues[1].number, 2);
        assert_eq!(issues[1].title, "Issue 2");
    }
}

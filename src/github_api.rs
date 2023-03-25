use reqwest::{
    header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT},
    Client,
};

use crate::models::UserRepositorySizeModel;

const USER_AGENT_VALUE: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36";
const ACCEPT_VALUE: &str = "application/vnd.github+json";
const X_GITHUB_API_VERSION: &str = "X-GitHub-Api-Version";
const X_GITHUB_API_VERSION_VALUE: &str = "2022-11-28";

pub struct GitHubApi {
    auth_token: String,
}

impl GitHubApi {
    pub fn new(token: &str) -> Self {
        let auth_token = format!("Bearer {token}");

        Self { auth_token }
    }

    fn get_url(username: &str) -> String {
        format!("https://api.github.com/users/{username}/repos?sort=pushed&per_page=5&type=public")
    }

    fn get_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, USER_AGENT_VALUE.parse().unwrap());
        headers.insert(ACCEPT, ACCEPT_VALUE.parse().unwrap());
        headers.insert(AUTHORIZATION, self.auth_token.parse().unwrap());
        headers.insert(
            X_GITHUB_API_VERSION,
            X_GITHUB_API_VERSION_VALUE.parse().unwrap(),
        );

        headers
    }

    pub async fn fetch_data(&self, username: &str) -> Vec<UserRepositorySizeModel> {
        let url = Self::get_url(username);

        let client = Client::new();
        let resp = client
            .get(url)
            .headers(self.get_headers())
            .send()
            .await
            .unwrap();

        if !resp.status().is_success() {
            // println!("{:?}", resp.text().await?);
        }

        let resp_text = resp.text().await.unwrap();
        serde_json::from_str(&resp_text).unwrap()
    }
}

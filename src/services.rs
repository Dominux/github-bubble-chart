use std::cmp::Reverse;

use crate::github_api::GitHubApi;

pub struct UserRepositoryService {
    gh: GitHubApi,
}

impl UserRepositoryService {
    pub fn new(token: &str) -> Self {
        Self {
            gh: GitHubApi::new(token),
        }
    }

    pub async fn get_chart(&self, username: &str) {
        // fetching user's repos from github
        let mut repos_list = self.gh.fetch_data(username).await;

        // sorting them
        // repos_list.sort_unstable_by_key(|r| Reverse(r.size));
        println!("{:#?}", repos_list);
    }
}

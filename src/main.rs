use github_api::GitHubApi;

mod github_api;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env!("GITHUB_ACCESS_TOKEN");

    let data = GitHubApi::new(token).fetch_data("Dominux").await;

    println!("{:#?}", data);
    Ok(())
}

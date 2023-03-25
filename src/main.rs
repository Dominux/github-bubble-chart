use services::UserRepositoryService;

mod chart_builder;
mod common;
mod controllers;
mod github_api;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env!("GITHUB_ACCESS_TOKEN");

    UserRepositoryService::new(token).get_chart("Dominux").await;

    Ok(())
}

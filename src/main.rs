use mdpbackend::config::application::AppConfig;
use mdpbackend::infrastructure::db::connection;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::build().unwrap();
    let conn_pool = connection(&config.db_config).await;
    
    mdpbackend::run(config, conn_pool).await
}

use super::config::Config;
use super::product::Product;
// use std::collections::HashMap;
// use reqwest;

#[allow(dead_code)]
fn update_zunkasite_product(product: &Product, config: &Config) {
    println!("product: {}", product);
    println!("config: {}", config.db_filename);
}

#[allow(dead_code)]
// #[tokio::main]
async fn get_all_allnations_products(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(&format!(
            "{}/setup/products/allnations",
            &config.zunkasite_host
        ))
        .basic_auth(&config.zunkasite_user, Some(&config.zunkasite_pass))
        .send()
        .await
        .unwrap();
    println!("{}", response.status());
    println!("Status: {}", response.status());

    let body = response.text().await.unwrap();
    println!("Body:\n\n{}", body);

    Ok(())
}

mod test {
    // #[test]
    #[tokio::test]
    async fn test_get_all_allnations_products() {
        let config = super::super::config::Config::new();
        // tokio::run(super::get_all_allnations_products(&config).await);
        super::get_all_allnations_products(&config).await.unwrap();
        println!("After function");
    }
}

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
#[tokio::main]
async fn get_all_allnations_products(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get("https://httpbin.org/basic-auth/user/passwd")
        .basic_auth(&config.zunkasite_user, Some(&config.zunkasite_pass))
        .send()
        .await
        .unwrap();
    println!("{}", response.status());

    let res = reqwest::get(&format!(
        "{}/setup/products/allnations",
        &config.zunkasite_host
    ))
    .await
    .unwrap();

    println!("Status: {}", res.status());

    let body = res.text().await.unwrap();
    println!("Body:\n\n{}", body);

    Ok(())
}

mod test {
    // #[test]
    #[tokio::test]
    async fn test_get_all_allnations_products() {
        let config = super::super::config::Config::new();
        super::get_all_allnations_products(&config).await;
        println!("After function");
    }
}

// #[tokio::main]
// async fn get_all_allnations_products(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
// // let resp = reqwest::get(&format!(
// // "{}/setup/products/allnations",
// // &config.zunkasite_host
// // ))
// // .await?
// // .json::<HashMap<String, String>>()
// // .await?;
// // println!("{:#?}", resp);
// // Ok(())

// let res = reqwest::get(&format!(
// "{}/setup/products/allnations",
// &config.zunkasite_host
// ))
// .await?;
// println!("Status: {}", res.status());

// let body = res.text().await?;
// println!("Body:\n\n{}", body);

// Ok(())
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
// let resp = reqwest::get("https://httpbin.org/ip")
// .await?
// .json::<HashMap<String, String>>()
// .await?;
// println!("{:#?}", resp);
// Ok(())
// }

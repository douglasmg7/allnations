use super::config::Config;
use super::product::Product;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
// // use std::collections::HashMap;
// use reqwest;

// struct AllnationsProductsInfo {
// zunka_site_id: String,
// code: String,
// dealer_price: i64,
// dealer_product_active: bool,
// stock: i32,
// }

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
    // .json()
    // .unwrap();

    println!("Status: {}", response.status());
    let body = response.text().await.unwrap();
    println!("Body:\n\n{}", body);

    Ok(())
}

#[allow(dead_code)]
async fn get_all_allnations_products_codes_from_zunka(
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    // let response = reqwest::Client::new()
    // .get(&format!(
    // "{}/setup/products/allnations",
    // &config.zunkasite_host
    // ))
    // .basic_auth(&config.zunkasite_user, Some(&config.zunkasite_pass))
    // .send()
    // .await
    // .unwrap();
    // // .json()
    // // .unwrap();

    // println!("Status: {}", response.status());
    // let body = response.text().await.unwrap();

    #[derive(Deserialize, Serialize, Debug)]
    #[allow(non_snake_case)]
    struct ProductTemp {
        dealerProductId: String,
    }

    // impl Eq for Products {}
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        [
            {   
                "id":"5f50f2e711a5c24a18524d81",
                "dealerProductId":"0072079",
                "dealerProductActive":true,
                "dealerProductPrice":3300,
                "storeProductQtd":22
            },
            {
                "id":"5f5228665cea0b08536459c0",
                "dealerProductId":"0074532",
                "dealerProductActive":true,
                "dealerProductPrice":30,
                "storeProductQtd":113
            }
        ]"#;

    // Deserialize to vec product.
    let products: Vec<ProductTemp> = serde_json::from_str(data).unwrap();

    let mut products_code = std::collections::HashSet::new();

    for product in products.iter() {
        products_code.insert(product.dealerProductId.clone());
    }

    // // Access parts of the data by indexing with square brackets.
    // println!("p: {:?}", p);
    // // Fisrt value.
    // println!("p[0]: {}", v[0]);

    // Dealer product id.
    // println!("p[0].id: {}", p[0].dealerProductId);
    // println!("Body:\n\n{}", body);

    Ok(products_code)
}

mod test {
    // #[tokio::test(core_threads = 1)]
    #[tokio::test]
    #[ignore]
    async fn test_get_all_allnations_products() {
        let config = super::super::config::Config::new();
        super::get_all_allnations_products(&config).await.unwrap();
    }

    #[tokio::test]
    async fn get_all_allnations_products_codes_from_zunka() {
        let products = super::get_all_allnations_products_codes_from_zunka()
            .await
            .unwrap();
        println!("products: {:?}", products);
    }

    #[test]
    #[ignore]
    fn serde_typed_array() {
        use serde::{Deserialize, Serialize};

        #[allow(non_snake_case)]
        #[derive(Deserialize, Serialize, Debug)]
        struct Products {
            dealerProductId: String,
        }
        // impl Eq for Products {}
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"
        [
            {   
                "id":"5f50f2e711a5c24a18524d81",
                "dealerProductId":"0072079",
                "dealerProductActive":true,
                "dealerProductPrice":3300,
                "storeProductQtd":22
            },
            {
                "id":"5f5228665cea0b08536459c0",
                "dealerProductId":"0074532",
                "dealerProductActive":true,
                "dealerProductPrice":30,
                "storeProductQtd":113
            }
        ]"#;

        // Parse the string of data into serde_json::Value.
        // let p: Vec<Products> = serde_json::from_str(data).unwrap();
        let p: Vec<Products> = serde_json::from_str(data).unwrap();

        // // Access parts of the data by indexing with square brackets.
        println!("p: {:?}", p);
        // // Fisrt value.
        // println!("p[0]: {}", v[0]);

        // Dealer product id.
        println!("p[0].dealerProductId: {}", p[0].dealerProductId);
    }

    #[test]
    #[ignore]
    fn serde_typed_hash_set() {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize, PartialEq, Debug, Hash, Eq)]
        struct Products {
            id: String,
        }
        // impl Eq for Products {}
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"
        [
            {   
                "id":"5f50f2e711a5c24a18524d81",
                "dealerProductId":"0072079",
                "dealerProductActive":true,
                "dealerProductPrice":3300,
                "storeProductQtd":22
            },
            {
                "id":"5f5228665cea0b08536459c0",
                "dealerProductId":"0074532",
                "dealerProductActive":true,
                "dealerProductPrice":30,
                "storeProductQtd":113
            }
        ]"#;

        // Parse the string of data into serde_json::Value.
        // let p: Vec<Products> = serde_json::from_str(data).unwrap();
        let p: std::collections::HashSet<Products> = serde_json::from_str(data).unwrap();

        // // Access parts of the data by indexing with square brackets.
        println!("p: {:?}", p.len());
        println!("p: {:?}", p);
        // // Fisrt value.
        // println!("v[0]: {}", v[0]);

        // Dealer product id.
        // println!("v[0].id: {}", v[0]["id"]);
    }

    #[test]
    #[ignore]
    fn serde_untyped() {
        // use serde_json::{Result, Value};
        use serde_json::Value;
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"
        [
            {   
                "id":"5f50f2e711a5c24a18524d81",
                "dealerProductId":"0072079",
                "dealerProductActive":true,
                "dealerProductPrice":3300,
                "storeProductQtd":22
            },
            {
                "id":"5f5228665cea0b08536459c0",
                "dealerProductId":"0074532",
                "dealerProductActive":true,
                "dealerProductPrice":30,
                "storeProductQtd":113
            }
        ]"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data).unwrap();

        // // Access parts of the data by indexing with square brackets.
        // println!("v: {}", v);
        // // Fisrt value.
        // println!("v[0]: {}", v[0]);

        // Dealer product id.
        println!("v[0].id: {}", v[0]["id"]);
    }

    #[test]
    #[ignore]
    fn serde_untyped_example() {
        // use serde_json::{Result, Value};
        use serde_json::Value;
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        // Parse the string of data into serde_json::Value.
        let v: Value = serde_json::from_str(data).unwrap();

        // Access parts of the data by indexing with square brackets.
        println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    }
}

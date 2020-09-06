use super::config::Config;
use super::product::Product;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
// // use std::collections::HashMap;
// use reqwest;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ProductResponse {
    dealerProductId: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ProductUpdate {
    storeProductId: String,
    dealerProductActive: bool,
    dealerProductPrice: f64,
    storeProductQtd: u32,
}

#[allow(dead_code)]
pub async fn get_all_allnations_products_codes_from_zunka(
    config: &Config,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let products_response = reqwest::Client::new()
        .get(&format!(
            "{}/setup/products/allnations",
            &config.zunkasite_host
        ))
        .basic_auth(&config.zunkasite_user, Some(&config.zunkasite_pass))
        .send()
        .await
        .unwrap()
        .json::<Vec<ProductResponse>>()
        .await
        .unwrap();

    let mut products_code = std::collections::HashSet::new();

    for product in products_response.iter() {
        products_code.insert(product.dealerProductId.clone());
    }

    Ok(products_code)
}

#[allow(dead_code)]
pub async fn update_allnations_products_from_zunka(
    config: Config,
    product: Product,
) -> Result<bool, Box<dyn std::error::Error + Send>> {
    let product_update = ProductUpdate {
        storeProductId: product.zunka_product_id.clone(),
        dealerProductActive: product.active && product.availability,
        dealerProductPrice: f64::from(product.price_sale) / 100.,
        storeProductQtd: product.stock_qty,
    };

    let response = reqwest::Client::new()
        .post(&format!("{}/setup/product/update", &config.zunkasite_host))
        .basic_auth(&config.zunkasite_user, Some(&config.zunkasite_pass))
        .json(&product_update)
        .send()
        .await
        .unwrap();

    log::debug!(
            "Updated zunkasite product, code: {:?}, zunka_product_id: {:?}, description: {:?}, category: {:?}",
            product.code, product.zunka_product_id, product.description, product.category
        );

    Ok(response.status().is_success())
}

mod test {
    #[tokio::test]
    #[ignore]
    async fn get_all_allnations_products_codes_from_zunka() {
        let products = super::get_all_allnations_products_codes_from_zunka(
            &super::super::config::Config::new(),
        )
        .await
        .unwrap();
        println!("products: {:?}", products);
    }

    // #[tokio::test(core_threads = 1)]
    #[tokio::test]
    #[ignore]
    async fn update_allnations_products_from_zunka() {
        let config = super::super::config::Config::new();
        let mut product = super::super::Product::new();
        // product.zunka_product_id = "6f5228665cea0b08536459c0".to_string();
        product.zunka_product_id = "5f5228665cea0b08536459c0".to_string();
        product.active = true;
        product.availability = true;
        product.stock_qty = 32;
        product.price_sale = 200000;

        let product2 = product.clone();
        let config2 = config.clone();

        let mut joins = Vec::new();

        joins.push(tokio::task::spawn(
            super::update_allnations_products_from_zunka(config, product),
        ));

        joins.push(tokio::task::spawn(
            super::update_allnations_products_from_zunka(config2, product2),
        ));

        // println!("*** End ***");

        for join in joins {
            let r = join.await.unwrap().unwrap();
            assert!(r);
            // println!("r: {}", r);
        }
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
                "dealerProductId":"0072071",
                "dealerProductActive":true,
                "dealerProductPrice":3300,
                "storeProductQtd":22
            },
            {
                "id":"5f5228665cea0b08536459c0",
                "dealerProductId":"0074533",
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
                "dealerProductId":"0072071",
                "dealerProductActive":true,
                "dealerProductPrice":3300,
                "storeProductQtd":22
            },
            {
                "id":"5f5228665cea0b08536459c0",
                "dealerProductId":"0074533",
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
                "dealerProductId":"0072071",
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

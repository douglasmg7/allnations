// use category::Category;
use chrono::{FixedOffset, Utc};
use log::{debug, error, info};
use product::Product;
// use std::collections::{HashMap, HashSet};
use std::panic;

pub mod category;
pub mod config;
pub mod logger;
pub mod macros;
pub mod product;
pub mod zunkasite;

#[derive(Clone)]
pub enum RunMode {
    Prod(),
    Dev(),
    Test(),
}

// Run.
pub async fn run<T: std::io::Read>(
    config: &config::Config,
    file: T,
) -> Result<(), Box<dyn std::error::Error>> {
    // Log panic as error.
    panic::set_hook(Box::new(|info| {
        error!("{}", info);
    }));

    // Log configuration.
    config.log();

    // Init db connection.
    let mut conn = rusqlite::Connection::open(&config.db_filename).unwrap();

    // Import products from xml.
    let mut products = Product::from_xml(file);

    // Process products.
    let changed_products = process_products(&mut products, &mut conn);

    // Update zunkasite products.
    update_zunka_site(config, &changed_products).await;

    Ok(())
}

/// Proccess products.
pub fn process_products(
    products: &mut Vec<Product>,
    conn: &mut rusqlite::Connection,
) -> Vec<Product> {
    let mut total_count = 0;
    let mut changed_count = 0;
    let mut new_count = 0;
    let mut old_count = 0;
    let mut not_changed_count = 0;

    let now = now!();

    // Changed products, not new.
    let mut changed_products = Vec::new();

    for product in products.iter_mut() {
        total_count += 1;

        let db_product = Product::get_one(&conn, &product.code);
        // New product.
        if db_product.is_none() {
            product.created_at = now;
            product.changed_at = now;
            product.save(&conn);
            new_count += 1;
        // info!("New product {}", product.code);
        }
        // Existing product.
        else {
            let db_product = db_product.unwrap();
            // Same product.
            if product == &db_product {
                not_changed_count += 1;
                continue;
            }
            // Product is older.
            if product.timestamp < db_product.timestamp {
                old_count += 1;
                // warn!("Product {} have a timestamp older or equal, current product: {}, pretended new product: {}", product.code, db_product.timestamp, product.timestamp);
                continue;
            }
            // Product changed.
            let diff = product.diff(&db_product);
            if diff.len() != 0 {
                debug!("Diff for product {}\n{}", product.code, diff);
            }
            changed_count += 1;
            let tx = conn.transaction().unwrap();
            // Save on history.
            db_product.save_history(&tx);
            // Update product.
            product.created_at = db_product.created_at.clone();
            product.changed_at = now.clone();
            product.zunka_product_id = db_product.zunka_product_id.clone();
            product.update(&tx);
            tx.commit().unwrap();
            // Changed products.
            changed_products.push(product.clone())
        }
    }
    info!("**********  Products  **********");
    info!("    Processed: {}", total_count);
    if new_count > 0 {
        info!("          New: {}", new_count);
    }
    if changed_count > 0 {
        info!("      Changed: {}", changed_count);
    }
    if old_count > 0 {
        info!("Old timestamp: {}", old_count);
    }
    if not_changed_count > 0 {
        info!("  Not changed: {}", not_changed_count);
    }

    changed_products
}

// Update zunka site with changed products.
async fn update_zunka_site(config: &config::Config, changed_products: &Vec<Product>) {
    // Join tasks.
    let mut joins = Vec::new();
    // Updated allnations products.
    for product in changed_products {
        if !product.zunka_product_id.is_empty() {
            joins.push(tokio::task::spawn(
                zunkasite::update_allnations_products_from_zunka(config.clone(), product.clone()),
            ));
        }
    }
    // Wait all.
    for join in joins {
        let res = join.await.unwrap().unwrap();
        println!("res: {}", res);
    }
}

// Formated price from u32.
#[allow(dead_code)]
fn formated_price_from_u32(num: u32) -> String {
    let s = num.to_string().chars().rev().collect::<String>();
    let mut result = String::new();
    for (i, c) in s.char_indices() {
        match i {
            2 => {
                result.push('.');
            }
            5 => {
                result.push(',');
            }
            8 => {
                result.push(',');
            }
            _ => {}
        }
        result.push(c);
    }
    match result.len() {
        1 => {
            result.push_str("0.0");
        }
        2 => {
            result.push_str(".0");
        }
        3 => {
            result.push_str("0");
        }
        _ => {}
    }
    let result = result.chars().rev().collect::<String>();
    format!("R$ {}", result)
}

mod test {
    #[test]
    fn formated_price() {
        assert_eq!(super::formated_price_from_u32(1), "R$ 0.01");
        assert_eq!(super::formated_price_from_u32(12), "R$ 0.12");
        assert_eq!(super::formated_price_from_u32(123), "R$ 1.23");
        assert_eq!(super::formated_price_from_u32(23456789), "R$ 234,567.89");
        assert_eq!(super::formated_price_from_u32(123456789), "R$ 1,234,567.89");
    }

    #[tokio::test]
    async fn run() {
        // use super::{category::Category, config::Config, logger, product::Product};
        use super::{config::Config, logger, product::Product};
        use std::{fs::File, io::BufReader};

        // Config.
        let config = Box::leak(Box::new(Config::new()));

        // Connection.
        let conn = rusqlite::Connection::open(&config.db_filename).unwrap();

        // Init log.
        logger::init(&config.run_mode).unwrap();

        // Clean db.
        // Category::remove_all(&conn);
        Product::remove_all(&conn);
        Product::remove_all_history(&conn);

        // Add category to use.
        // let category_to_use = Category::new("ARMAZENAMENTO", 2, true);
        // category_to_use.save(&conn);

        let mut path_a = std::env::current_dir().unwrap();
        path_a.push("xml");
        let mut path_b = path_a.clone();
        path_a.push("allnations_products_a.xml");
        path_b.push("allnations_products_b.xml");

        // Run using file a.
        let file = File::open(path_a).unwrap();
        assert!(super::run(&config, BufReader::new(file)).await.is_ok());
        let product = Product::get_one(&conn, "0070495").unwrap();
        assert_eq!(product.price_sale, 206136);
        assert_eq!(Product::get_all_hsitory(&conn).len(), 0);

        // Update zunka id to test update zunka product.
        // product.zunka_product_id = "5f55305461cacd48fc506464".to_string();
        // product.update(&conn);

        // Run using file b.
        let file = File::open(path_b).unwrap();
        assert!(super::run(&config, BufReader::new(file)).await.is_ok());
        let product = Product::get_one(&conn, "0070495").unwrap();
        assert_eq!(product.price_sale, 207136);
        assert_eq!(Product::get_all_hsitory(&conn)[0].price_sale, 206136);
        // assert_eq!(Category::get_all(&conn).len(), 39);
    }
}
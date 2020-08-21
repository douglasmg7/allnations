use category::Category;
use chrono::{FixedOffset, Utc};
use log::{error, info};
use product::Product;
use std::collections::{HashMap, HashSet};
use std::panic;

pub mod category;
pub mod config;
pub mod logger;
pub mod macros;
pub mod product;

pub enum RunMode {
    Prod(),
    Dev(),
    Test(),
}

// Run.
pub fn run<T: std::io::Read>(
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
    process_products(&mut products, &mut conn);

    // Selected categories.
    let selected_categories_array = Category::get_all_selected(&conn);
    let mut selected_categories = HashSet::new();
    for category in selected_categories_array.iter() {
        selected_categories.insert(category.name.clone());
    }

    // Create categories.
    let mut categories = HashMap::<String, Category>::new();
    let products = Product::get_all(&conn);
    let mut products_in_use_count = 0;
    let mut min_price = u32::MAX;
    let mut max_price = u32::MIN;
    for product in products.iter() {
        let name = Category::name_from_text(&product.category);
        match categories.get_mut(&name) {
            Some(category) => {
                category.products_qty += 1;
            }
            None => {
                categories.insert(name.clone(), Category::new(&product.category, 0, false));
            }
        }
        // Products in use.
        if selected_categories.contains(&name) {
            products_in_use_count += 1;
        }
        // Min price.
        if product.price_sale < min_price {
            min_price = product.price_sale;
        }

        // Max price.
        if product.price_sale > max_price {
            max_price = product.price_sale;
        }
    }

    // Update categories.
    for category in categories.values_mut() {
        if selected_categories.contains(&category.name) {
            category.selected = true;
        }
        match Category::get_one(&conn, &category.name) {
            Some(db_category) => {
                if db_category != *category {
                    category.update(&conn);
                }
            }
            None => {
                category.save(&conn);
            }
        }
    }

    println!("Total products: {}", products.len());
    println!("Used products: {}", products_in_use_count);
    println!("Min price products: {}", formated_price_from_u32(min_price));
    println!("Max price products: {}", formated_price_from_u32(max_price));
    println!("Total category: {}", categories.len());
    println!("Selected categories: {}", selected_categories.len());

    Ok(())
}

/// Proccess products.
pub fn process_products(products: &mut Vec<Product>, conn: &mut rusqlite::Connection) {
    let mut total_count = 0;
    let mut changed_count = 0;
    let mut new_count = 0;
    let mut old_count = 0;

    let now = now!();

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
            // Product is older.
            if product.timestamp <= db_product.timestamp {
                old_count += 1;
                // warn!("Product {} have a timestamp older or equal, current product: {}, pretended new product: {}", product.code, db_product.timestamp, product.timestamp);
                continue;
            }
            // Product changed.
            if product != &db_product {
                // Save product on history and update product.
                let tx = conn.transaction().unwrap();
                // Save on history.
                db_product.save_history(&tx);
                // Update product.
                product.created_at = db_product.created_at;
                product.changed_at = now;
                product.zunka_product_id = db_product.zunka_product_id;
                product.update(&tx);
                // Update zunkasite product.
                // todo
                tx.commit().unwrap();
                changed_count += 1;
                // info!("Product {} updated", product.code);
            }
        }
    }
    info!("Total products processed: {}", total_count);
    info!("New products: {}", new_count);
    info!("Changed products: {}", changed_count);
    info!("Old products: {}", old_count);
}

// Formated price from u32.
fn formated_price_from_u32(num: u32) -> String {
    let s = num.to_string().chars().rev().collect::<String>();
    let mut result = String::new();
    for (i, c) in s.char_indices() {
        result.push(c);
        match i {
            1 => {
                result.push('.');
            }
            4 => {
                result.push(',');
            }
            7 => {
                result.push(',');
            }
            _ => {}
        }
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
        assert_eq!(super::formated_price_from_u32(123456789), "R$ 1,234,567.89");
    }

    #[test]
    fn run() {
        use super::{category::Category, config::Config, logger, product::Product};
        use std::{fs::File, io::BufReader};

        // Config.
        let config = Box::leak(Box::new(Config::new()));

        // Connection.
        let conn = rusqlite::Connection::open(&config.db_filename).unwrap();

        // Init log.
        logger::init(&config.run_mode).unwrap();

        // Clean db.
        Category::remove_all(&conn);
        Product::remove_all(&conn);
        Product::remove_all_history(&conn);

        // Add category to use.
        let category_to_use = Category::new("ARMAZENAMENTO", 2, true);
        category_to_use.save(&conn);

        let mut path_a = std::env::current_dir().unwrap();
        path_a.push("xml");
        let mut path_b = path_a.clone();
        path_a.push("allnations_products_a.xml");
        path_b.push("allnations_products_b.xml");

        // Run using file a.
        let file = File::open(path_a).unwrap();
        assert!(super::run(&config, BufReader::new(file)).is_ok());
        let product = Product::get_one(&conn, "0070495").unwrap();
        assert_eq!(product.price_sale, 206136);
        assert_eq!(Product::get_all_hsitory(&conn).len(), 0);

        // Run using file b.
        let file = File::open(path_b).unwrap();
        assert!(super::run(&config, BufReader::new(file)).is_ok());
        let product = Product::get_one(&conn, "0070495").unwrap();
        assert_eq!(product.price_sale, 207136);
        assert_eq!(Product::get_all_hsitory(&conn)[0].price_sale, 206136);
        assert_eq!(Category::get_all(&conn).len(), 39);
    }
}
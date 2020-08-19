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

    // // Import products from xml.
    // let stdin = std::io::stdin();
    // let mut products = Product::from_xml(stdin.lock());

    // Import products from xml.
    let mut products = Product::from_xml(file);

    // Get all categories and selected categories.
    let categories_array = Category::get_all(&conn);
    let mut categories = HashMap::<String, &Category>::new();
    let mut selected_categories = HashSet::new();
    for category in categories_array.iter() {
        categories.insert(category.name.clone(), category);
        if category.selected {
            selected_categories.insert(category.name.clone());
        }
    }

    // Process products.
    let new_categories = process_products(
        &mut products,
        &selected_categories,
        &config.filter,
        &mut conn,
    );

    // Update categories.
    for (name, new_category) in new_categories.iter() {
        match categories.get(name) {
            Some(category) => {
                if *category != new_category {
                    new_category.update(&conn);
                }
            }
            None => {
                new_category.save(&conn);
            }
        }
    }

    Ok(())
}

/// Proccess products.
pub fn process_products(
    products: &mut Vec<Product>,
    selected_categories: &HashSet<String>,
    filter: &config::Filter,
    conn: &mut rusqlite::Connection,
) -> HashMap<String, Category> {
    let mut new_categories: HashMap<String, Category> = HashMap::new();

    let mut min_price = std::u32::MAX;
    let mut max_price = std::u32::MIN;

    let mut cut_by_max_price_count = 0;
    let mut cut_by_min_price_count = 0;
    let mut cut_by_category_count = 0;

    let mut total_products_count = 0;
    let mut used_products_count = 0;

    let now = now!();

    for product in products.iter_mut() {
        total_products_count += 1;
        // Filter by category.
        let name = Category::name_from_text(&product.category);
        // Inc not selected categories.
        if selected_categories.get(&name).is_none() {
            cut_by_category_count += 1;
            match new_categories.get_mut(&name) {
                Some(category) => {
                    category.products_qtd += 1;
                }
                None => {
                    new_categories.insert(name, Category::new(&product.category, 1, false));
                }
            }
            continue;
        }
        // Inc selected categories.
        match new_categories.get_mut(&name) {
            Some(category) => {
                category.products_qtd += 1;
            }
            None => {
                new_categories.insert(name, Category::new(&product.category, 1, true));
            }
        }
        // Filter by min price.
        if product.price_sale < filter.min_price {
            cut_by_min_price_count += 1;
            continue;
        }
        // Filter by max price.
        if product.price_sale > filter.max_price {
            cut_by_max_price_count += 1;
            continue;
        }
        // Used products count.
        used_products_count += 1;
        // Min price.
        if product.price_sale < min_price {
            min_price = product.price_sale;
        }
        // Max price.
        if product.price_sale > max_price {
            max_price = product.price_sale;
        }

        let db_product = Product::get_one(&conn, &product.code);
        // New product.
        if db_product.is_none() {
            product.created_at = now;
            product.changed_at = now;
            product.save(&conn);
        }
        // Existing product.
        else {
            let db_product = db_product.unwrap();
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
            }
        }
    }
    info!(
        "Using {} products from {}",
        used_products_count, total_products_count
    );
    info!("Min price: {}", formated_price_from_u32(min_price));
    info!("Max price: {}", formated_price_from_u32(max_price));
    info!(
        "Products cutted by min price({}): {}",
        formated_price_from_u32(filter.min_price),
        cut_by_min_price_count
    );
    info!(
        "Products cutted by max price({}): {}",
        formated_price_from_u32(filter.max_price),
        cut_by_max_price_count
    );
    info!(
        "Products cutted by categories filter: {}",
        cut_by_category_count
    );
    info!(
        "Using {} categories from {}",
        selected_categories.len(),
        new_categories.len()
    );
    new_categories
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

    // #[test]
    // fn run() {
    // use std::fs::File;
    // use std::io::BufReader;

    // let mut conn = rusqlite::Connection::open(&super::Config::new().db_filename).unwrap();
    // // let now = super::super::now!();

    // // Remove all categories.
    // Category::remove_all(&conn);

    // let mut path_a = std::env::current_dir().unwrap();
    // path_a.push("xml");
    // let mut path_b = path_a.clone();
    // path_a.push("allnations_products_a.xml");
    // path_b.push("allnations_products_b.xml");
    // // println!("path_a: {:?}", path_a);

    // // Run using file a.
    // let file = File::open(path_a).unwrap();
    // let config = super::config::Config::new();
    // assert!(super::run(config, BufReader::new(file)).is_ok());

    // // Run using file b.
    // let file = File::open(path_b).unwrap();
    // let config = super::config::Config::new();
    // assert!(super::run(config, BufReader::new(file)).is_ok());
    // }
}
use category::Category;
use chrono::{FixedOffset, Utc};
use log::{debug, error};
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
pub fn run(config: config::Config) -> Result<(), Box<dyn std::error::Error>> {
    // Log panic as error.
    panic::set_hook(Box::new(|info| {
        error!("{}", info);
    }));

    // Log configuration.
    config.log();

    // Init db connection.
    let conn = rusqlite::Connection::open(&config.db_filename).unwrap();

    // Import products from xml.
    let stdin = std::io::stdin();
    let products = Product::from_xml(stdin.lock());

    // Get all selected categories.
    let selected_categories_array = Category::get_all_selected(&conn);
    let mut selected_categories = HashMap::new();
    for sel_category in selected_categories_array.iter() {
        selected_categories.insert(&sel_category.name, sel_category);
    }

    // Insert product.
    products[0].save(&conn);
    debug!("Products quanatity: {}", products.len());
    Product::get_all(&conn);
    Ok(())
}

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn process_products(
    products: &mut Vec<Product>,
    selected_categories: &HashMap<String, Category>,
    filter: &config::Filter,
    conn: &rusqlite::Connection,
) {
    let mut min_price = std::i32::MAX;
    let mut max_price = std::i32::MIN;

    let mut cut_by_max_price_count = 0;
    let mut cut_by_min_price_count = 0;
    let mut cut_by_category_count = 0;

    // All categories text by products quantity.
    let mut all_categories_text = HashMap::<String, i32>::new();
    // Selected categories text by products quantity.
    let mut selected_categories_text = HashSet::<String>::new();
    // let categories_in_use = HashMap::new();

    let mut total_products_count = 0;
    let mut used_products_count = 0;

    let now = now!();

    for product in products.iter_mut() {
        total_products_count += 1;
        // Add category.
        all_categories_text.insert(
            product.category.clone(),
            all_categories_text.get(&product.category).unwrap_or(&0) + 1,
        );
        // Filter by category.
        if selected_categories
            .get(
                &product
                    .category
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join("_")
                    .to_uppercase(),
            )
            .is_none()
        {
            cut_by_category_count += 1;
            continue;
        }
        // Add category in use.
        selected_categories_text.insert(product.category.clone());
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
        if product.price_sale < max_price {
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
            // Product changed.
            let db_product = db_product.unwrap();
            if product != &db_product {
                product.created_at = db_product.created_at;
                product.changed_at = now;
                product.zunka_product_id = db_product.zunka_product_id;
                product.update(&conn);
                // Update zunkasite product.
                // todo
            }
        }
    }
    println!(
        "Using {} products from {}",
        used_products_count, total_products_count
    );
    println!(
        "Products cutted by min price({}): {}",
        min_price, cut_by_min_price_count
    );
    println!(
        "Products cutted by max price({}): {}",
        max_price, cut_by_max_price_count
    );
    println!(
        "Products cutted by categories filter: {}",
        cut_by_category_count
    );
    println!(
        "Using {} categories from {}",
        selected_categories_text.len(),
        all_categories_text.len()
    );
    // updateDBCategories(&mCategoryAll)
    // Update all categories.
    // todo
}
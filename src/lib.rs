use log::{debug, error};
use std::collections::HashMap;
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
    let products = product::products_from_xml(stdin.lock());

    // Insert product.
    products[0].save(&conn);
    debug!("Products quanatity: {}", products.len());
    product::Product::get_all(&conn).unwrap();
    Ok(())
}

#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn process_products(products: Vec<product::Product>) {
    let mut min_price = std::i32::MAX;
    let mut max_price = std::i32::MIN;

    let mut qtd_cut_by_max_price = 0;
    let mut qtd_cut_by_min_price = 0;
    let mut qtd_cut_by_categ_filter = 0;
    let mut qtd_cut_by_error = 0;

    let mut categories_all = HashMap::<String, i32>::new();
    // let categories_in_use = HashMap::new();

    let mut total_qtd_products = 0;
    let mut used_qtd_products = 0;

    for product in products.iter() {
        total_qtd_products += 1;
        // Map all categories.
        categories_all.insert(
            product.category.clone(),
            categories_all.get(&product.category).unwrap_or(&0) + 1,
        );
        // Filter by categorie.
    }
}
use log::{debug, error};
use std::collections::HashMap;
use std::panic;

pub mod category;
pub mod config;
pub mod db;
pub mod logger;
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

    // Init db.
    let db = db::Db::new(&config.db_filename);

    // Import products from xml.
    let stdin = std::io::stdin();
    let products = product::products_from_xml(stdin.lock());

    // Insert product.
    db.insert_product(&products[0]);

    // for p in products {
    // println!("{}", p);
    // }

    debug!("Products quanatity: {}", products.len());

    db.select_all_products().unwrap();

    Ok(())
}

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
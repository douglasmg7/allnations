use log::{debug, error};
use std::panic;

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
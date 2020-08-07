pub mod config;
pub mod db;
pub mod product;

pub enum RunMode {
    Prod(),
    Dev(),
    Test(),
}

// Run.
pub fn run(config: config::Config) -> Result<(), Box<dyn std::error::Error>> {
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

    println!("Products quanatity: {}", products.len());

    db.select_all_products().unwrap();

    Ok(())
}
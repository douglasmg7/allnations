pub mod config;
pub mod db;
pub mod product;

// Run.
pub fn run(config: config::Config) -> Result<(), Box<dyn std::error::Error>> {
    config.log();
    let _production = config.production;
    let db = db::Db::new(&config.db_filename);

    // Import products from xml.
    let stdin = std::io::stdin();
    let products = product::products_from_xml(stdin.lock());
    // for p in products {
    // println!("{}", p);
    // }
    println!("Products quanatity: {}", products.len());

    // Insert product.
    // allnations::db::insert_product(&products[0]);

    db.get_all_products().unwrap();

    Ok(())
}
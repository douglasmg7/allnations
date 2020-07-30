// use std::error::Error;

fn main() {
    // Set run mode.
    allnations::set_run_mode();

    // match allnations::db::insert_xml_product() {
    // Ok(()) => println!("Ok"),
    // Err(message) => println!("message: {}", message),
    // }

    // Import products from xml.
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let products = allnations::product::products_from_xml(stdin);
    println!("Products quanatity: {}", products.len());

    // Insert product.
    // allnations::db::insert_product(&products[0]);

    // println!("stmt: {}", allnations::product::STMT_PRODUCT_SELECT_ALL);
    allnations::db::print_test();
    allnations::db::get_all_products().unwrap();

    // for p in products {
    // println!("{}", p);
    // }

    // // Config.
    // let config = Config::new(env::args()).unwrap_or_else(|err| {
    // eprintln!("Problem parsin arguments: {}", err);
    // process::exit(1);
    // });

    // // Run.
    // if let Err(e) = minigrep::run(config) {
    // eprintln!("Application error: {}", e);
    // process::exit(1);
    // }
}

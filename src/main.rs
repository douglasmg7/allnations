fn main() {
    // Set run mode.
    allnations::set_run_mode();

    // Import products from xml.
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let products = allnations::xml::products_from_xml(stdin);
    println!("Products quanatity: {}", products.len());

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

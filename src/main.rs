// provides `try_next`
use futures::TryStreamExt;
use sqlx::Connection;
// use sqlx::prelude::*;
use sqlx::sqlite::SqliteRow;
// Need for row.try_get();
use sqlx::Row;
use std::env;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // Set run mode.
    allnations::set_run_mode();

    // let pool = SqlitePoolOptions::new()
    // .max_connections(5)
    // .connect(
    // &env::var("ZUNKA_ALLNATIONS_DB").expect("Environment variable ZUNKA_ALLNATIONS_DB"),
    // )
    // .await
    // .unwrap();
    let db_string = format!(
        "sqlite://{}",
        env::var("ZUNKA_ALLNATIONS_DB").expect("Environment variable ZUNKA_ALLNATIONS_DB")
    );

    println!("db_string: {}", db_string);
    let mut conn = sqlx::SqliteConnection::connect(&db_string).await?;

    // let _sql_a = sqlx::query("SELECT * FROM product").persistent(true);

    // let mut rows = sqlx::query("SELECT * FROM product").fetch(&mut conn);

    // // use futures::TryStreamExt;
    // while let Some(row) = rows.try_next().await? {
    // // map the row into a user-defined domain type
    // // use sqlx::Row;
    // let code: &str = row.try_get("code")?;
    // println!("code: {}", code);
    // }

    #[derive(sqlx::FromRow)]
    struct ProductA {
        code: String,
        description: String,
    }

    let mut stream = sqlx::query_as::<_, ProductA>("SELECT * FROM product").fetch(&mut conn);
    println!("stream: {:?}", stream.next.await);

    // let mut stream = sqlx::query("SELECT * FROM users")
    // .map(|row: SqliteRow| {
    // // map the row into a user-defined domain type
    // // let a = row.get(0);
    // Ok()
    // })
    // .fetch(&mut conn);

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
    // allnations::db::print_test();
    // allnations::db::get_all_products().unwrap();

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

    Ok(())
}

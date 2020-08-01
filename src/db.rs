// use super::*;
// use chrono::DateTime;

// macro_rules! t1 {
// () => {
// "
// code, description, timestamp, department, category, sub_category, maker, technical_description, url_image, part_number, ean, ncm,
// price_sale, price_without_st, icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, availability, origin,
// stock_origin, stock_qtd, created_at, changed_at
// "
// };
// }

// const SQL: &str = concat!("SELECT ", t1!(), "FROM product");

// const PRODUCT_FIELDS: &str = "
// code, description, timestamp, department, category, sub_category, maker, technical_description, url_image, part_number, ean, ncm,
// price_sale, price_without_st, icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, availability, origin,
// stock_origin, stock_qtd, created_at, changed_at
// ";

// // pub static STMT_PRODUCT_SELECT_ALL: &str = format!("SELECT {} FROM product", PRODUCT_FIELDS);
// pub const STMT_PRODUCT_SELECT_ALL: &str = concat!("a", "b");

// pub fn print_test() {
// println!("sql: {}", SQL);
// }

// pub fn insert_product(product: &product::Product) {
// let conn = rusqlite::Connection::open(DB_FILE.get().unwrap()).unwrap();
// let now = chrono::Local::now().to_rfc3339();
// conn.execute(
// "INSERT INTO product (
// code, description, timestamp, department, category, sub_category, maker, technical_description, url_image, part_number, ean, ncm,
// price_sale, price_without_st, icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, availability, origin,
// stock_origin, stock_qtd, created_at, changed_at)
// VALUES (?1, ?2, ?3, ?4, ?5, ?6,  ?7 , ?8 , ?9 , ?10 , ?11 , ?12 , ?13 , ?14 , ?15 , ?16 , ?17 , ?18 , ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27)",
// rusqlite::params![
// product.code, product.description, product.timestamp.to_rfc3339(), product.department, product.category, product.sub_category, product.maker,
// product.technical_description, product.url_image, product.part_number, product.ean, product.ncm, product.price_sale,
// product.price_without_st, product.icms_st_taxation, product.warranty_month, product.length_mm, product.width_mm,
// product.height_mm, product.weight_g, product.active, product.availability, product.origin,
// product.stock_origin, product.stock_qtd, now, now],
// )
// .unwrap();
// }

// pub fn get_all_products() -> Result<(), rusqlite::Error> {
// let conn = rusqlite::Connection::open(DB_FILE.get().unwrap())?;
// // let mut stmt = conn.prepare("SELECT code, description FROM product")?;
// let mut stmt = conn.prepare(&format!("SELECT {} FROM product", PRODUCT_FIELDS))?;
// let product_iter = stmt.query_map(rusqlite::params![], |row| {
// // let mut product = product::Product::new();
// // product.code = row.get(0)?;
// // product.description = row.get(1)?;
// Ok(product::Product {
// code: row.get(0)?,
// description: row.get(1)?,
// // timestamp: DateTime::parse_from_rfc3339(row.get(2)?.to_string()).unwrap(),
// timestamp: DateTime::parse_from_rfc3339("0000-01-02T00:00:00-03:00").unwrap(),
// department: row.get(3)?,
// category: row.get(4)?,
// sub_category: row.get(5)?,
// maker: row.get(5)?,
// technical_description: row.get(6)?,
// url_image: row.get(7)?,
// part_number: row.get(8)?,
// ean: row.get(9)?,
// ncm: row.get(10)?,
// price_sale: row.get(11)?,
// price_without_st: row.get(12)?,
// icms_st_taxation: row.get(13)?,
// warranty_month: row.get(14)?,
// length_mm: row.get(15)?,
// width_mm: row.get(16)?,
// height_mm: row.get(17)?,
// weight_g: row.get(18)?,
// active: row.get(19)?,
// availability: row.get(20)?,
// origin: row.get(21)?,
// stock_origin: row.get(21)?,
// stock_qtd: row.get(22)?,
// created_at: DateTime::parse_from_rfc3339("0000-01-02T00:00:00-03:00").unwrap(),
// changed_at: DateTime::parse_from_rfc3339("0000-01-02T00:00:00-03:00").unwrap(),
// removed_at: DateTime::parse_from_rfc3339("0000-01-02T00:00:00-03:00").unwrap(),
// checked_at: DateTime::parse_from_rfc3339("0000-01-02T00:00:00-03:00").unwrap(),
// // created_at: DateTime::parse_from_rfc3339(row.get(23)?.to_string()).unwrap(),
// // changed_at: DateTime::parse_from_rfc3339(row.get(24)?.to_string()).unwrap(),
// // removed_at: DateTime::parse_from_rfc3339(row.get(25)?.to_string()).unwrap(),
// // checked_at: DateTime::parse_from_rfc3339(row.get(26)?.to_string()).unwrap(),
// })
// })?;

// for product in product_iter {
// println!("{}", product.unwrap());
// }
// Ok(())
// }

// pub fn get_all_products() -> Result<(), rusqlite::Error> {
// let conn = rusqlite::Connection::open(DB_FILE.get().unwrap())?;
// let mut stmt = conn.prepare("SELECT code, description FROM product")?;
// let product_iter = stmt.query_map(rusqlite::params![], |row| {
// let mut product = xml::Product::new();
// product.code = row.get(0)?;
// product.description = row.get(1)?;
// Ok(product)
// })?;

// for product in product_iter {
// println!("{}", product.unwrap());
// }
// Ok(())
// }

// let conn = rusqlite::Connection::open(DB_FILE.get().unwrap()).unwrap();
// t mut stmt = conn.prepare("SELECT code, description FROM product").unwrap();
// let product_iter = stmt.query_map(params![], |row| {
// Ok(Product {
// id: row.get(0)?,
// name: row.get(1)?,
// time_created: row.get(2)?,
// data: row.get(3)?,
// })
// })?;

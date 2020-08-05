use super::*;
use chrono::{Utc, DateTime, FixedOffset, SecondsFormat};

// This is a simple macro named `say_hello`.
#[allow(unused_macros)]
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!");
    };
}

const PRODUCT_FIELDS: &str = " \
code, description, timestamp, department, category, sub_category, maker, technical_description, url_image, part_number, ean, ncm, \
price_sale, price_without_st, icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, availability, origin, \
stock_origin, stock_qtd, created_at, changed_at, removed_at, checked_at ";

pub struct Db {
    conn: rusqlite::Connection,
}

impl Db {
    pub fn new(db_filename: &str) -> Db {
        Db {
            conn: rusqlite::Connection::open(db_filename).unwrap(),
        }
    }

    pub fn insert_product(&self, product: &product::Product) {
        let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600)).to_rfc3339_opts(SecondsFormat::Secs, false);
        self.conn.execute(
            "INSERT INTO product (
            code, description, timestamp, department, category, sub_category, maker, technical_description, url_image, part_number, ean, ncm,
            price_sale, price_without_st, icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, availability, origin,
            stock_origin, stock_qtd, created_at, changed_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6,  ?7 , ?8 , ?9 , ?10 , ?11 , ?12 , ?13 , ?14 , ?15 , ?16 , ?17 , ?18 , ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27)",
            rusqlite::params![product.code, product.description, product.timestamp.to_rfc3339(), product.department, product.category,
            product.sub_category, product.maker, product.technical_description, product.url_image, product.part_number, product.ean,
            product.ncm, product.price_sale, product.price_without_st, product.icms_st_taxation, product.warranty_month, product.length_mm, 
            product.width_mm, product.height_mm, product.weight_g, product.active, product.availability, product.origin, product.stock_origin, 
            product.stock_qtd, now, now],
        ).unwrap();
    }

    pub fn get_all_products(&self) -> Result<(), rusqlite::Error> {
        // let mut stmt = conn.prepare("SELECT code, description FROM product")?;
        let mut stmt = self
            .conn
            .prepare(&format!("SELECT {} FROM product", PRODUCT_FIELDS))?;

        let product_iter = stmt.query_map(rusqlite::params![], |row| {
            // let timestamp: DateTime<Utc> = row.get(2)?;
            // timestamp: timestamp.with_timezone(&FixedOffset::west(3600 * 3)),

            // let a = row.get::<_, String>(2)?;
            // println!("a: {}", a);

            // let a = row.get::<_, DateTime<Utc>>(2)?.with_timezone(&FixedOffset::west(3600 * 3));
            // println!("a: {}", a);

            Ok(product::Product {
                code: row.get(0)?,
                description: row.get(1)?,
                // timestamp: timestamp.with_timezone(&FixedOffset::west(3600 * 3)),
                timestamp: row.get::<_, DateTime<Utc>>(2)?.with_timezone(&FixedOffset::west(3600 * 3)),
                department: row.get(3)?,
                category: row.get(4)?,
                sub_category: row.get(5)?,
                maker: row.get(6)?,
                technical_description: row.get(7)?,
                url_image: row.get(8)?,
                part_number: row.get(9)?,
                ean: row.get(10)?,
                ncm: row.get(11)?,
                price_sale: row.get(12)?,
                price_without_st: row.get(13)?,
                icms_st_taxation: row.get(14)?,
                warranty_month: row.get(15)?,
                length_mm: row.get(16)?,
                width_mm: row.get(17)?,
                height_mm: row.get(18)?,
                weight_g: row.get(19)?,
                active: row.get(20)?,
                availability: row.get(21)?,
                origin: row.get(22)?,
                stock_origin: row.get(23)?,
                stock_qtd: row.get(24)?,
                created_at: row.get::<_, DateTime<Utc>>(25)?.with_timezone(&FixedOffset::west(3600 * 3)),
                changed_at: row.get::<_, DateTime<Utc>>(26)?.with_timezone(&FixedOffset::west(3600 * 3)),
                removed_at: row.get::<_, DateTime<Utc>>(27)?.with_timezone(&FixedOffset::west(3600 * 3)),
                checked_at: row.get::<_, DateTime<Utc>>(28)?.with_timezone(&FixedOffset::west(3600 * 3)),
            })
        })?;

        for product in product_iter {
            let product = product.unwrap();
            println!("{}", product);
            // println!("code: {}", product.code);
            // println!("timestamp: {}", product.timestamp);
            // println!("timestamp: {}", product.timestamp);
            // println!("timestamp: {}", product.timestamp);
            // println!("timestamp: {}", product.timestamp);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_fields(){
        println!("[{}]", PRODUCT_FIELDS);
        // assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    pub fn test_macro(){
        // say_hello!();
        if cfg!(test) {
            println!("Running in test");
        } else {
            println!("Running in productionk");
        }
    }

    #[test]
    pub fn insert_product(){
        // Configuration.
        let config = super::config::Config::new();
        let db = db::Db::new(&config.db_filename);
        db.insert_product(&product::Product::new());
    }
}

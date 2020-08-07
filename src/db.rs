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

// #[allow(unused_macros)]
// macro_rules! named_params {
    // ( $( $x:expr ),* ) => {
        // {
            // $(
                // println!("{}", $x);
                // $x;
            // )*
        // }
    // };
// }

// named_params!("asdadsfasdf");

const PRODUCT_FIELDS: &str = " \
zunka_product_id, code, description, timestamp, department, category, sub_category, maker, \
technical_description, url_image, part_number, ean, ncm, price_sale, price_without_st, \
icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, \
availability, origin, stock_origin, stock_qtd, created_at, changed_at, removed_at, checked_at ";

pub struct Db {
    conn: rusqlite::Connection,
    sql_select_product_by_code: String,
    sql_select_all_products: String,
    sql_update_product_by_code: String,
}

impl Db {
    pub fn new(db_filename: &str) -> Db {
        let product_fields  = "zunka_product_id, code, description, timestamp, department, category, sub_category, maker, \
technical_description, url_image, part_number, ean, ncm, price_sale, price_without_st, \
icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, \
availability, origin, stock_origin, stock_qtd, created_at, changed_at, removed_at, checked_at"
            .to_string()
            .split(",")
            .map(|x| format!("{}", x.trim()))
            .collect::<Vec<_>>()
            .join(", ");

        // Select product by code.
        let select_product_by_code = format!(r#"SELECT {} FROM product WHERE code = :code"#, product_fields);
        // let select_product_by_code = format!(r#"SELECT {} FROM product WHERE code = "?""#, product_fields);
        // println!("select_product_by_code: [{}]", select_product_by_code);

        // Select all products.
        let select_all_products = format!("SELECT {} FROM product", product_fields);
        // println!("select_all_products: [{}]", select_all_products);

        // Product fields formated to update.
        let product_fields_formated_to_update = product_fields.split(",")
            .map(|x| format!("{0} = :{0}", x.trim()))
            .collect::<Vec<_>>()
            .join(", ");

        // Update product by code.
        let update_product_by_code = format!(r#"UPDATE product SET {} WHERE code = :code"#, product_fields_formated_to_update);
        // println!("update_product_by_code: [{}]", update_product_by_code);

        Db {
            conn: rusqlite::Connection::open(db_filename).unwrap(),
            sql_select_product_by_code: select_product_by_code,
            sql_select_all_products: select_all_products,
            sql_update_product_by_code: update_product_by_code,
        }
    }

    // Delete all products.
    pub fn delete_all_products(&self) {
        self.conn.execute(
            "DELETE FROM PRODUCT",
            rusqlite::NO_PARAMS,
        ).unwrap();
    }

    // Insert one product.
    pub fn insert_product(&self, product: &product::Product) {
        // let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600)).to_rfc3339_opts(SecondsFormat::Secs, false);
        self.conn.execute( &format!("INSERT INTO product ({}) VALUES 
                (?1, ?2, ?3, ?4, ?5, ?6,  ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30)",
                PRODUCT_FIELDS),
            rusqlite::params![product.zunka_product_id, product.code, product.description, product.timestamp.to_rfc3339(), product.department, product.category,
            product.sub_category, product.maker, product.technical_description, product.url_image, product.part_number, product.ean,
            product.ncm, product.price_sale, product.price_without_st, product.icms_st_taxation, product.warranty_month, product.length_mm, 
            product.width_mm, product.height_mm, product.weight_g, product.active, product.availability, product.origin, product.stock_origin, product.stock_qtd, 
            product.created_at.to_rfc3339_opts(SecondsFormat::Secs, false), 
            product.changed_at.to_rfc3339_opts(SecondsFormat::Secs, false), 
            product.checked_at.to_rfc3339_opts(SecondsFormat::Secs, false), 
            product.removed_at.to_rfc3339_opts(SecondsFormat::Secs, false)],
        ).unwrap();
    }

    // Update product by code.
    pub fn update_product_by_code(&self, product: &product::Product) {
        // let mut stmt = self.conn.prepare("UPDATE product SET technical_description = :technical_description WHERE code = :code").unwrap();
        // stmt.execute_named(rusqlite::named_params!{":code": product.code, ":technical_description": product.technical_description}).unwrap();

        let mut stmt = self.conn.prepare(&self.sql_update_product_by_code).unwrap();
        stmt.execute_named(&[
            (":zunka_product_id", &product.zunka_product_id), 
            (":code", &product.code), 
            (":description", &product.description), 
            (":timestamp", &product.timestamp), 
            (":department", &product.department), 
            (":category", &product.category), 
            (":sub_category", &product.sub_category), 
            (":maker", &product.maker), 
            (":technical_description", &product.technical_description),
            (":url_image", &product.url_image), 
            (":part_number", &product.part_number), 
            (":ean", &product.ean), 
            (":ncm", &product.ncm), 
            (":price_sale", &product.price_sale), 
            (":price_without_st", &product.price_without_st), 
            (":icms_st_taxation", &product.icms_st_taxation), 
            (":warranty_month", &product.warranty_month), 
            (":length_mm", &product.length_mm), 
            (":width_mm", &product.width_mm), 
            (":height_mm", &product.height_mm), 
            (":weight_g", &product.weight_g), 
            (":active", &product.active), 
            (":availability", &product.availability), 
            (":origin", &product.origin), 
            (":stock_origin", &product.stock_origin), 
            (":stock_qtd", &product.stock_qtd), 
            (":created_at", &product.created_at), 
            (":changed_at", &product.changed_at), 
            (":checked_at", &product.checked_at), 
            (":removed_at", &product.removed_at), 
        ]).unwrap();
    }

    // Get all products.
    pub fn select_all_products(&self) -> Option<Vec<product::Product>> {
        // let mut stmt = conn.prepare("SELECT code, description FROM product")?;
        let mut stmt = self
            .conn
            .prepare(&self.sql_select_all_products).unwrap();

        let product_iter = stmt.query_map(rusqlite::params![], |row| {
            // let timestamp: DateTime<Utc> = row.get(2)?;
            // timestamp: timestamp.with_timezone(&FixedOffset::west(3600 * 3)),

            // let a = row.get::<_, String>(2)?;
            // println!("a: {}", a);

            // let a = row.get::<_, DateTime<Utc>>(2)?.with_timezone(&FixedOffset::west(3600 * 3));
            // println!("a: {}", a);

            Ok(product::Product {
                zunka_product_id: row.get(0).unwrap(),
                code: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                // timestamp: timestamp.with_timezone(&FixedOffset::west(3600 * 3)),
                timestamp: row.get::<_, DateTime<Utc>>(3).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                department: row.get(4).unwrap(),
                category: row.get(5).unwrap(),
                sub_category: row.get(6).unwrap(),
                maker: row.get(7).unwrap(),
                technical_description: row.get(8).unwrap(),
                url_image: row.get(9).unwrap(),
                part_number: row.get(10).unwrap(),
                ean: row.get(11).unwrap(),
                ncm: row.get(12).unwrap(),
                price_sale: row.get(13).unwrap(),
                price_without_st: row.get(14).unwrap(),
                icms_st_taxation: row.get(15).unwrap(),
                warranty_month: row.get(16).unwrap(),
                length_mm: row.get(17).unwrap(),
                width_mm: row.get(18).unwrap(),
                height_mm: row.get(19).unwrap(),
                weight_g: row.get(20).unwrap(),
                active: row.get(21).unwrap(),
                availability: row.get(22).unwrap(),
                origin: row.get(23).unwrap(),
                stock_origin: row.get(24).unwrap(),
                stock_qtd: row.get(25).unwrap(),
                created_at: row.get::<_, DateTime<Utc>>(26).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                changed_at: row.get::<_, DateTime<Utc>>(27).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                removed_at: row.get::<_, DateTime<Utc>>(28).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                checked_at: row.get::<_, DateTime<Utc>>(29).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
            })
        }).unwrap();

        let mut products = Vec::new();
        for product in product_iter {
            products.push(product.unwrap());
        }
        Some(products)
    }

    // Select product by code.
    pub fn select_product_by_code(&self, code: &str) -> Option<product::Product> {
        let mut stmt = self.conn.prepare(&self.sql_select_product_by_code).unwrap();
        let mut rows = stmt.query_named(&[(":code", &code)]).unwrap();

        let row = rows.next().unwrap();
        match row {
            None => None,
            Some(row) => {
                Some(product::Product {
                    zunka_product_id: row.get(0).unwrap(),
                    code: row.get(1).unwrap(),
                    description: row.get(2).unwrap(),
                    // timestamp: timestamp.with_timezone(&FixedOffset::west(3600 * 3)),
                    timestamp: row.get::<_, DateTime<Utc>>(3).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                    department: row.get(4).unwrap(),
                    category: row.get(5).unwrap(),
                    sub_category: row.get(6).unwrap(),
                    maker: row.get(7).unwrap(),
                    technical_description: row.get(8).unwrap(),
                    url_image: row.get(9).unwrap(),
                    part_number: row.get(10).unwrap(),
                    ean: row.get(11).unwrap(),
                    ncm: row.get(12).unwrap(),
                    price_sale: row.get(13).unwrap(),
                    price_without_st: row.get(14).unwrap(),
                    icms_st_taxation: row.get(15).unwrap(),
                    warranty_month: row.get(16).unwrap(),
                    length_mm: row.get(17).unwrap(),
                    width_mm: row.get(18).unwrap(),
                    height_mm: row.get(19).unwrap(),
                    weight_g: row.get(20).unwrap(),
                    active: row.get(21).unwrap(),
                    availability: row.get(22).unwrap(),
                    origin: row.get(23).unwrap(),
                    stock_origin: row.get(24).unwrap(),
                    stock_qtd: row.get(25).unwrap(),
                    created_at: row.get::<_, DateTime<Utc>>(26).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                    changed_at: row.get::<_, DateTime<Utc>>(27).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                    removed_at: row.get::<_, DateTime<Utc>>(28).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                    checked_at: row.get::<_, DateTime<Utc>>(29).unwrap().with_timezone(&FixedOffset::west(3600 * 3)),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn print_fields(){
        // println!("[{}]", PRODUCT_FIELDS);
        // let s = FA.clone();
        // println!("[{}]", s);
        let config = super::config::Config::new();
        let _db = db::Db::new(&config.db_filename);
        // println!("sql: [{}]", db.sql);
    }

    // #[test]
    // pub fn test_macro(){

        // // let a = named_params!["aaa", "bbb"];
        // // println!("macro: {:?}", a);

        // // // say_hello!();
        // // if cfg!(test) {
            // // println!("Running in test");
        // // } else {
            // // println!("Running in productionk");
        // // }
    // }

    #[test]
    pub fn crud_product(){
        // Configuration.
        let config = super::config::Config::new();
        let db = db::Db::new(&config.db_filename);

        // Delete all products.
        db.delete_all_products();
        let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600));

        // Insert product.
        // let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600)).to_rfc3339_opts(SecondsFormat::Secs, false);
        let mut product_insert = product::Product{
            zunka_product_id: "123456789012345678901234".to_string(), 
            code: "0074632".to_string(),
            description: "IMPRESSORA EPSON TERMICA NAO FISCAL TM-T20X SERIAL/USB- C31CH26031".to_string(),
            timestamp: now - chrono::Duration::days(7),
            department: "AUTOMAÇÃO".to_string(),
            category: "IMPRESSORA NÃO FISCAL".to_string(),
            sub_category: "IMPRESSORA NÃO FISCAL".to_string(),
            maker: "EPSON".to_string(),
            technical_description: r#"A econômica impressora térmica TM-T20X da Epson é versátil, possui alta confiabilidade e funções fáceis de usar, como carga rápida de papel (drop-
in), guilhotina inclusa e muito mais.

Especificações:
SERIAL/USB
Alguns dos benefícios da TM-T20X são: baixo consumo de energia, monitoramento de status da impressora, além de contadores de manutenção, que são excelentes ferramentas de
 controle.
Garantia Epson - 3 anos balcão¹.
Confiabilidade - MCBF de 60 milhões de linhas, MTBF de 360.000 horas e guilhotina de 1,5 milhão de cortes.
Alta velocidade de impressão - Até 200 mm/s.
Funções fáceis de usar - Carga rápida de papel, cortador automático e LEDs de status da impressora.
Ecoamigável - Atende aos requisitos RoHS."#.to_string(),
            url_image: "http://images.allnations.com.br/imagens/produtos/imagemSite.aspx?h=196&l=246&src=0074632".to_string(),
            part_number: "C31CH26031".to_string(),
            ean: "010343952010".to_string(),
            ncm: "84433239".to_string(),
            price_sale: 55612,
            price_without_st: 55612,
            icms_st_taxation: false,
            warranty_month: 24,
            length_mm: 240,
            width_mm: 190,
            height_mm: 240,
            weight_g: 2290,
            active: true,
            availability: true,
            origin: "4 - NACIONAL - CONF. PROCESSO PRODUTIVO".to_string(),
            stock_origin: "ES".to_string(),
            stock_qtd: 42,
            created_at: now.clone(),
            changed_at: now.clone(),
            checked_at: product::ZERO_TIME.clone(), 
            removed_at: product::ZERO_TIME.clone(),
        };
        db.insert_product(&product_insert);
        db.insert_product(&product::Product::new());

        // Select product by code.
        let product_select = db.select_product_by_code(&product_insert.code).unwrap();
        // println!("productInsert: {}", product_insert.zunka_product_id);
        // println!("productSelect: {}", product_select.zunka_product_id);
        assert_eq!(product_insert, product_select);

        // Select all products.
        let products = db.select_all_products().unwrap();
        assert!(products.len() > 1);

        // Update product.
        product_insert.technical_description = "asdf".to_string();
        db.update_product_by_code(&product_insert);
        let product_select = db.select_product_by_code(&product_insert.code).unwrap();
        assert_eq!(product_insert, product_select);
    }
}

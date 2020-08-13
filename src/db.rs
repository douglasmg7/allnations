use super::*;
use chrono::{DateTime, FixedOffset, Utc};

const PRODUCT_FIELDS: &str =
    "zunka_product_id, code, description, timestamp, department, category, sub_category, maker, \
technical_description, url_image, part_number, ean, ncm, price_sale, price_without_st, \
icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, \
availability, origin, stock_origin, stock_qtd, created_at, changed_at, removed_at, checked_at";

#[allow(unused_macros)]
macro_rules! product_example {
    ($now: ident) => {
        product::Product{
            zunka_product_id: "123456789012345678901234".to_string(),
            code: "0074632".to_string(),
            description: "IMPRESSORA EPSON TERMICA NAO FISCAL TM-T20X SERIAL/USB- C31CH26031".to_string(),
            timestamp: $now - chrono::Duration::days(7),
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
            created_at: $now.clone(),
            changed_at: $now.clone(),
            checked_at: product::ZERO_TIME.clone(),
            removed_at: product::ZERO_TIME.clone(),
        };
    }
}

// Create execute named product for rusqlite statement.
macro_rules! stmt_execute_named_product {
    ($stmt: ident, $product: ident) => {
        $stmt
            .execute_named(&[
                (":zunka_product_id", &$product.zunka_product_id),
                (":code", &$product.code),
                (":description", &$product.description),
                (":timestamp", &$product.timestamp),
                (":department", &$product.department),
                (":category", &$product.category),
                (":sub_category", &$product.sub_category),
                (":maker", &$product.maker),
                (":technical_description", &$product.technical_description),
                (":url_image", &$product.url_image),
                (":part_number", &$product.part_number),
                (":ean", &$product.ean),
                (":ncm", &$product.ncm),
                (":price_sale", &$product.price_sale),
                (":price_without_st", &$product.price_without_st),
                (":icms_st_taxation", &$product.icms_st_taxation),
                (":warranty_month", &$product.warranty_month),
                (":length_mm", &$product.length_mm),
                (":width_mm", &$product.width_mm),
                (":height_mm", &$product.height_mm),
                (":weight_g", &$product.weight_g),
                (":active", &$product.active),
                (":availability", &$product.availability),
                (":origin", &$product.origin),
                (":stock_origin", &$product.stock_origin),
                (":stock_qtd", &$product.stock_qtd),
                (":created_at", &$product.created_at),
                (":changed_at", &$product.changed_at),
                (":checked_at", &$product.checked_at),
                (":removed_at", &$product.removed_at),
            ])
            .unwrap();
    };
}

// Create product from a row.
macro_rules! product_from_row {
    // let timestamp: DateTime<Utc> = row.get(2)?;
    // timestamp: timestamp.with_timezone(&FixedOffset::west(3600 * 3)),

    // let a = row.get::<_, String>(2)?;
    // println!("a: {}", a);

    // let a = row.get::<_, DateTime<Utc>>(2)?.with_timezone(&FixedOffset::west(3600 * 3));
    // println!("a: {}", a);
    ($row: ident) => {
        product::Product {
            zunka_product_id: $row.get(0).unwrap(),
            code: $row.get(1).unwrap(),
            description: $row.get(2).unwrap(),
            // timestamp: timestamp.with_timezone(&FixedOffset::west(3600 * 3)),
            timestamp: $row
                .get::<_, DateTime<Utc>>(3)
                .unwrap()
                .with_timezone(&FixedOffset::west(3600 * 3)),
            department: $row.get(4).unwrap(),
            category: $row.get(5).unwrap(),
            sub_category: $row.get(6).unwrap(),
            maker: $row.get(7).unwrap(),
            technical_description: $row.get(8).unwrap(),
            url_image: $row.get(9).unwrap(),
            part_number: $row.get(10).unwrap(),
            ean: $row.get(11).unwrap(),
            ncm: $row.get(12).unwrap(),
            price_sale: $row.get(13).unwrap(),
            price_without_st: $row.get(14).unwrap(),
            icms_st_taxation: $row.get(15).unwrap(),
            warranty_month: $row.get(16).unwrap(),
            length_mm: $row.get(17).unwrap(),
            width_mm: $row.get(18).unwrap(),
            height_mm: $row.get(19).unwrap(),
            weight_g: $row.get(20).unwrap(),
            active: $row.get(21).unwrap(),
            availability: $row.get(22).unwrap(),
            origin: $row.get(23).unwrap(),
            stock_origin: $row.get(24).unwrap(),
            stock_qtd: $row.get(25).unwrap(),
            created_at: $row
                .get::<_, DateTime<Utc>>(26)
                .unwrap()
                .with_timezone(&FixedOffset::west(3600 * 3)),
            changed_at: $row
                .get::<_, DateTime<Utc>>(27)
                .unwrap()
                .with_timezone(&FixedOffset::west(3600 * 3)),
            removed_at: $row
                .get::<_, DateTime<Utc>>(28)
                .unwrap()
                .with_timezone(&FixedOffset::west(3600 * 3)),
            checked_at: $row
                .get::<_, DateTime<Utc>>(29)
                .unwrap()
                .with_timezone(&FixedOffset::west(3600 * 3)),
        }
    };
}

// Create category from a row.
macro_rules! category_from_row {
    ($row: ident) => {
        category::Category {
            name: $row.get(0).unwrap(),
            text: $row.get(1).unwrap(),
            products_qtd: $row.get(2).unwrap(),
            selected: $row.get(3).unwrap(),
        }
    };
}

pub struct Db {
    conn: rusqlite::Connection,
    // Product.
    sql_insert_product: String,
    sql_update_product_by_code: String,
    sql_select_product_by_code: String,
    sql_select_all_products: String,
    // Category.
    sql_select_all_categories: String,
}

impl Db {
    pub fn new(db_filename: &str) -> Db {
        // Select product by code.
        let select_product_by_code = format!(
            r#"SELECT {} FROM product WHERE code = :code"#,
            PRODUCT_FIELDS
        );

        // Select all products.
        let select_all_products = format!("SELECT {} FROM product", PRODUCT_FIELDS);

        // Update product by code.
        let product_fields_update = PRODUCT_FIELDS
            .split(",")
            .map(|x| format!("{0} = :{0}", x.trim()))
            .collect::<Vec<_>>()
            .join(", ");
        let update_product_by_code = format!(
            r#"UPDATE product SET {} WHERE code = :code"#,
            product_fields_update
        );

        // Insert product.
        let product_fields_insert = PRODUCT_FIELDS
            .split(",")
            .map(|x| format!(":{0}", x.trim()))
            .collect::<Vec<_>>()
            .join(", ");
        let insert_product = format!(
            r#"INSERT INTO product ({}) VALUES ({})"#,
            PRODUCT_FIELDS, product_fields_insert
        );

        Db {
            conn: rusqlite::Connection::open(db_filename).unwrap(),
            // Product.
            sql_insert_product: insert_product,
            sql_update_product_by_code: update_product_by_code,
            sql_select_product_by_code: select_product_by_code,
            sql_select_all_products: select_all_products,
            // Category.
            sql_select_all_categories: "SELECT name, text, products_qtd, selected {} FROM category"
                .to_string(),
        }
    }

    /******************************************************
     * PRODUCT
     *******************************************************/
    // Delete all products.
    pub fn delete_all_products(&self) {
        self.conn
            .execute("DELETE FROM product", rusqlite::NO_PARAMS)
            .unwrap();
    }

    // Insert one product.
    pub fn insert_product(&self, product: &product::Product) {
        // let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600)).to_rfc3339_opts(SecondsFormat::Secs, false);
        let mut stmt = self.conn.prepare(&self.sql_insert_product).unwrap();
        stmt_execute_named_product!(stmt, product);
    }

    // Update product by code.
    pub fn update_product_by_code(&self, product: &product::Product) {
        let mut stmt = self.conn.prepare(&self.sql_update_product_by_code).unwrap();
        stmt_execute_named_product!(stmt, product);
    }

    // Select product by code.
    pub fn select_product_by_code(&self, code: &str) -> Option<product::Product> {
        let mut stmt = self.conn.prepare(&self.sql_select_product_by_code).unwrap();
        let mut rows = stmt.query_named(&[(":code", &code)]).unwrap();

        let row = rows.next().unwrap();
        match row {
            None => None,
            Some(row) => Some(product_from_row!(row)),
        }
    }

    // Get all products.
    pub fn select_all_products(&self) -> Option<Vec<product::Product>> {
        // let mut stmt = conn.prepare("SELECT code, description FROM product")?;
        let mut stmt = self.conn.prepare(&self.sql_select_all_products).unwrap();

        let product_iter = stmt
            .query_map(rusqlite::params![], |row| Ok(product_from_row!(row)))
            .unwrap();

        let mut products = Vec::new();
        for product in product_iter {
            products.push(product.unwrap());
        }
        Some(products)
    }

    /******************************************************
     * CAETORY
     *******************************************************/
    // Delete all categories.
    pub fn delete_all_categories(&self) {
        self.conn
            .execute("DELETE FROM category", rusqlite::NO_PARAMS)
            .unwrap();
    }

    // Get all categories.
    pub fn select_all_categories(&self) -> Option<Vec<category::Category>> {
        // let mut stmt = conn.prepare("SELECT code, description FROM product")?;
        let mut stmt = self.conn.prepare(&self.sql_select_all_categories).unwrap();

        let categories_iter = stmt
            .query_map(rusqlite::params![], |row| Ok(category_from_row!(row)))
            .unwrap();

        let mut categories = Vec::new();
        for category in categories_iter {
            categories.push(category.unwrap());
        }
        Some(categories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_fields() {
        let config = super::config::Config::new();
        let _db = db::Db::new(&config.db_filename);
    }

    #[test]
    pub fn crud_product() {
        // Configuration.
        let config = super::config::Config::new();
        let db = db::Db::new(&config.db_filename);

        // Delete all products.
        db.delete_all_products();

        // Insert product.
        let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600));
        // let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600)).to_rfc3339_opts(SecondsFormat::Secs, false);
        let mut product_insert = product_example!(now);
        db.insert_product(&product_insert);
        db.insert_product(&product::Product::new());

        // Select product by code.
        let product_select = db.select_product_by_code(&product_insert.code).unwrap();
        assert_eq!(product_insert, product_select);

        // Select all products.
        let products = db.select_all_products().unwrap();
        assert!(products.len() > 1);

        // Update product.
        product_insert.technical_description = "asdf".to_string();
        db.update_product_by_code(&product_insert);

        // Select product by code.
        let product_select = db.select_product_by_code(&product_insert.code).unwrap();
        assert_eq!(product_insert, product_select);
    }
}

use chrono::{DateTime, FixedOffset, Utc, SecondsFormat};

use lazy_static;
use std::fmt;

const PRODUCT_FIELDS: &str =
    "zunka_product_id, code, description, timestamp, department, category, sub_category, maker, \
technical_description, url_image, part_number, ean, ncm, price_sale, price_without_st, \
icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, \
availability, origin, stock_origin, stock_qtd, created_at, changed_at, removed_at, checked_at";

// const ZERO_TIME: &str = "0001-01-01T03:00:00-03:00";
lazy_static::lazy_static! {
    pub static ref ZERO_TIME: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("0001-01-01T03:00:00-03:00").unwrap();

    // Insert.
    static ref  PRODUCT_FIELDS_INSERT: String = PRODUCT_FIELDS
        .split(",")
        .map(|x| format!(":{0}", x.trim()))
        .collect::<Vec<_>>()
        .join(", ");
    static ref SQL_INSERT: String = format!(
        r#"INSERT INTO product ({}) VALUES ({})"#,
        PRODUCT_FIELDS, &*PRODUCT_FIELDS_INSERT // &* To go inside the wrap.
    );
    static ref SQL_INSERT_HIS: String = format!(
        r#"INSERT INTO product_history ({}) VALUES ({})"#,
        PRODUCT_FIELDS, &*PRODUCT_FIELDS_INSERT // &* To go inside the wrap.
    );
    
    // Select by code.
    static ref SQL_SELECT_BY_CODE: String = format!(
        r#"SELECT {} FROM product WHERE code = :code"#,
        PRODUCT_FIELDS
    );

    // Update product by code.
    static ref PRODUCT_FIELDS_UPDATE: String = PRODUCT_FIELDS
        .split(",")
        .map(|x| format!("{0} = :{0}", x.trim()))
        .collect::<Vec<_>>()
        .join(", ");
    static ref SQL_UPDATE_BY_CODE: String = format!(
        r#"UPDATE product SET {} WHERE code = :code"#,
        &*PRODUCT_FIELDS_UPDATE
    );
}

// Aldo product.
#[derive(Debug)]
pub struct Product {
    pub zunka_product_id: String,
    pub code: String,
    pub description: String,
    pub timestamp: DateTime<FixedOffset>,
    pub department: String,
    pub category: String,
    pub sub_category: String,
    pub maker: String,
    pub technical_description: String,
    pub url_image: String,
    pub part_number: String,
    pub ean: String,
    pub ncm: String,
    pub price_sale: u32,
    pub price_without_st: u32,
    pub icms_st_taxation: bool,
    pub warranty_month: u32,
    pub length_mm: u32,
    pub width_mm: u32,
    pub height_mm: u32,
    pub weight_g: u32,
    pub active: bool,
    pub availability: bool,
    pub origin: String,     // Nacional, importado, entre outros...
    pub stock_origin: String,   // RJ, SC or ES.
    pub stock_qtd: u32,
    pub created_at: DateTime<FixedOffset>,
    pub changed_at: DateTime<FixedOffset>,
    pub checked_at: DateTime<FixedOffset>,
    pub removed_at: DateTime<FixedOffset>,
}

impl Product {
    pub fn new() -> Self {
        let now = super::now!();
        Product {
            zunka_product_id: String::new(),
            code: "1111".to_string(),
            description: String::new(),
            timestamp: ZERO_TIME.clone(),
            department: String::new(),
            category: String::new(),
            sub_category: String::new(),
            maker: String::new(),
            technical_description: String::new(),
            url_image: String::new(),
            part_number: String::new(),
            ean: String::new(),
            ncm: String::new(),
            price_sale: 0,
            price_without_st: 0,
            icms_st_taxation: false,
            warranty_month: 0,
            length_mm: 0,
            width_mm: 0,
            height_mm: 0,
            weight_g: 0,
            active: false,
            availability: false,
            origin: String::new(),
            stock_origin: String::new(),
            stock_qtd: 0,
            created_at: now.clone(),
            changed_at: now.clone(),
            checked_at: ZERO_TIME.clone(),
            removed_at: ZERO_TIME.clone(),
        }
    }

    // Get one from db.
    pub fn get_one(conn: &rusqlite::Connection, code: &str) -> Option<Product> {
        let mut stmt = conn.prepare(&SQL_SELECT_BY_CODE).unwrap();
        let mut rows = stmt.query_named(&[(":code", &code)]).unwrap();

        let row = rows.next().unwrap();
        match row {
            None => None,
            Some(row) => Some(super::product_from_row!(row)),
        }
    }

    // Save on db.
    pub fn save(&self, conn: &rusqlite::Connection) {
        let mut stmt = conn.prepare(&SQL_INSERT).unwrap();
        super::stmt_execute_named_product!(stmt, self);
    }

    // Save history on db.
    pub fn save_history(&self, conn: &rusqlite::Connection) {
        let mut stmt = conn.prepare(&SQL_INSERT_HIS).unwrap();
        super::stmt_execute_named_product!(stmt, self);
    }

    // Update on db.
    pub fn update(&self, conn: &rusqlite::Connection) {
        let mut stmt = conn.prepare(&SQL_UPDATE_BY_CODE).unwrap();
        super::stmt_execute_named_product!(stmt, self);
    }

    // Remove all from db.
    pub fn remove_all(conn: &rusqlite::Connection) {
        conn.execute("DELETE FROM product", rusqlite::NO_PARAMS)
            .unwrap();
    }

    // Remove all from db.
    pub fn remove_all_history(conn: &rusqlite::Connection) {
        conn.execute("DELETE FROM product_history", rusqlite::NO_PARAMS)
            .unwrap();
    }

    // Get all from db.
    pub fn get_all(conn: &rusqlite::Connection) -> Vec<Product> {
        let mut stmt = conn
            .prepare(&format!("SELECT {} FROM product", PRODUCT_FIELDS))
            .unwrap();
        let iter = stmt
            .query_map(rusqlite::params![], |row| Ok(super::product_from_row!(row)))
            .unwrap();
        let mut products = Vec::new();
        for product in iter {
            products.push(product.unwrap());
        }
        products
    }

    // Get all from db history.
    pub fn get_all_hsitory(conn: &rusqlite::Connection) -> Vec<Product> {
        let mut stmt = conn
            .prepare(&format!("SELECT {} FROM product_history", PRODUCT_FIELDS))
            .unwrap();
        let iter = stmt
            .query_map(rusqlite::params![], |row| Ok(super::product_from_row!(row)))
            .unwrap();
        let mut products = Vec::new();
        for product in iter {
            products.push(product.unwrap());
        }
        products
    }

    // Read xml from stdin.
    pub fn from_xml<T: std::io::Read>(xml: T) -> Vec<Product> {
        // let stdin = io::stdin();
        // let handle = stdin.lock();
        let parser = xml::reader::EventReader::new(xml);
        // let mut depth = 0;
        let mut tag = String::new();
        let mut inside_product = false;
        let mut products: Vec<Product> = Vec::new();
        let mut product = Product::new();
        for e in parser {
            match e {
                Ok(xml::reader::XmlEvent::StartElement { name, .. }) => {
                    if name.local_name.eq("Produtos") {
                        if inside_product {
                            panic!("Already inside product!");
                        }
                        inside_product = true;
                    }
                    if inside_product {
                        tag = name.local_name;
                    }
                }
                Ok(xml::reader::XmlEvent::Characters(text)) => match tag.as_str() {
                    "TIMESTAMP" => {
                        product.timestamp = chrono::DateTime::parse_from_rfc3339(&text)
                            .expect(&format!("Invalid TIMESTAMP: {}", text));
                    }
                    "DEPARTAMENTO" => product.department = text.clone(),
                    "CATEGORIA" => product.category = text.clone(),
                    "SUBCATEGORIA" => product.sub_category = text.clone(),
                    "FABRICANTE" => product.maker = text.clone(),
                    "CODIGO" => product.code = text.clone(),
                    "DESCRICAO" => product.description = text.clone(),
                    "DESCRTEC" => product.technical_description = text.clone(),
                    "PARTNUMBER" => product.part_number = text.clone(),
                    "EAN" => product.ean = text.clone(),
                    "GARANTIA" => {
                        product.warranty_month = text
                            .parse::<u32>()
                            .expect(&format!("Invalid GARANTIA: {}", text));
                    }
                    "PESOKG" => {
                        product.weight_g = (1000.0
                            * text
                            .parse::<f32>()
                            .expect(&format!("Invalid PESOKG: {}", text)))
                            as u32;
                    }
                    "PRECOREVENDA" => {
                        product.price_sale = (100.0
                            * text
                            .parse::<f32>()
                            .expect(&format!("Invalid PRECOREVENDA: {}", text)))
                            as u32;
                    }
                    "PRECOSEMST" => {
                        product.price_without_st = (100.0
                            * text
                            .parse::<f32>()
                            .expect(&format!("Invalid PRECOSEMST: {}", text)))
                            as u32;
                    }
                    "DISPONIVEL" => product.availability = text == "1",
                    "URLFOTOPRODUTO" => product.url_image = text.clone(),
                    "ESTOQUE" => product.stock_origin = text.clone(),
                    "NCM" => product.ncm = text.clone(),
                    "LARGURA" => {
                        product.width_mm = (1000.0
                            * text
                            .parse::<f32>()
                            .expect(&format!("Invalid LARGURA: {}", text)))
                            as u32;
                    }
                    "ALTURA" => {
                        product.height_mm = (1000.0
                            * text
                            .parse::<f32>()
                            .expect(&format!("Invalid ALTURA: {}", text)))
                            as u32;
                    }
                    "PROFUNDIDADE" => {
                        product.length_mm = (1000.0
                            * text
                            .parse::<f32>()
                            .expect(&format!("Invalid PROFUNDIDADE: {}", text)))
                            as u32;
                    }
                    "ATIVO" => product.active = text == "1",
                    "SUBSTTRIBUTARIA" => product.icms_st_taxation = text == "1",
                    "ORIGEMPRODUTO" => product.origin = text.clone(),
                    "ESTOQUEDISPONIVEL" => {
                        product.stock_qtd = (text
                            .parse::<f32>()
                            .expect(&format!("Invalid ESTOQUEDISPONIVEL: {}", text)))
                            as u32
                    }
                    _ => {}
                },
                Ok(xml::reader::XmlEvent::EndElement { name }) => {
                    if name.local_name.eq("Produtos") {
                        products.push(product);
                        product = Product::new();
                        inside_product = false;
                    }
                }
                Err(e) => {
                    panic!("Parsing XML: {}", e);
                    // error!("{}", e);
                    // println!("Error: {}", e);
                    // break;
                }
                _ => {}
            }
        }
        products
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[product]\n\tcode: {}\n\tdescription: {}\n\ttimestamp: {}\n\tdepartment: {}\n\tcategory: {}\n\tsub_category: {}\n\tmaker: {}\n\ttechnical_description bytes count: {}\
            \n\turl_image: {}\n\tpart_number: {}\n\tean: {}\n\tncm: {}\
            \n\tprice_sale: {}\n\tprice_without_st: {}\n\ticms_st_taxation: {}\n\twarranty_month: {}\
            \n\tlength_mm: {}\n\twidth_mm: {}\n\theight_mm: {}\n\tweight_g: {}\
            \n\tactive: {}\n\tavailability: {}\n\torigin: {}\n\tstock_origin: {}\n\tstock_qtd: {}\
            \n\tcreated_at: {}\n\tchanged_at: {}\n\tremoved_at: {}\n\tchecked_at: {}",
            self.code, self.description, self.timestamp, self.department, self.category, self.sub_category, self.maker, self.technical_description.len(), 
            self.url_image, self.part_number, self.ean, self.ncm,
            self.price_sale, self.price_without_st, self.icms_st_taxation, self.warranty_month,
            self.length_mm, self.width_mm, self.height_mm, self.weight_g,
            self.active, self.availability, self.origin, self.stock_origin, self.stock_qtd,
            self.created_at, self.changed_at, self.removed_at, self.checked_at,
        )
    }
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
            self.zunka_product_id == other.zunka_product_id
            && self.code == other.code
            && self.description == other.description
            && self.timestamp == other.timestamp
            && self.department == other.department
            && self.category == other.category
            && self.sub_category == other.sub_category
            && self.maker == other.maker
            && self.technical_description == other.technical_description
            && self.url_image == other.url_image
            && self.part_number == other.part_number
            && self.ean == other.ean
            && self.ncm == other.ncm
            && self.price_sale == other.price_sale
            && self.price_without_st == other.price_without_st
            && self.icms_st_taxation == other.icms_st_taxation
            && self.warranty_month == other.warranty_month
            && self.length_mm == other.length_mm
            && self.width_mm == other.width_mm
            && self.height_mm == other.height_mm
            && self.weight_g == other.weight_g
            && self.active == other.active
            && self.availability == other.availability
            && self.origin == other.origin
            && self.stock_origin == other.stock_origin
            && self.stock_qtd == other.stock_qtd
            && self.created_at.to_rfc3339_opts(SecondsFormat::Secs, false) == other.created_at.to_rfc3339_opts(SecondsFormat::Secs, false)
            && self.changed_at.to_rfc3339_opts(SecondsFormat::Secs, false) == other.changed_at.to_rfc3339_opts(SecondsFormat::Secs, false)
            && self.checked_at.to_rfc3339_opts(SecondsFormat::Secs, false) == other.checked_at.to_rfc3339_opts(SecondsFormat::Secs, false)
            && self.removed_at.to_rfc3339_opts(SecondsFormat::Secs, false) == other.removed_at.to_rfc3339_opts(SecondsFormat::Secs, false)
    }
}

#[allow(unused_imports)]
mod test {
    use super::super::config::Config;
    use super::super::product_example;
    use super::*;

    #[test]
    fn crud() {
        let mut conn = rusqlite::Connection::open(&Config::new().db_filename).unwrap();
        let now = super::super::now!();

        ///////////////////////////////////////////////////////////////////////
        // PRODUCT
        ///////////////////////////////////////////////////////////////////////
        // Remove all.
        Product::remove_all(&conn);

        // Insert.
        Product::new().save(&conn);
        let product_example = product_example!(now);
        product_example.save(&conn);

        // Select by code.
        let product_from_db = Product::get_one(&conn, &product_example.code).unwrap();
        assert_eq!(product_example, product_from_db);

        // Update.
        let mut product_example = product_example!(now);
        product_example.technical_description = "asdf".to_string();
        product_example.update(&conn);
        let product_from_db = Product::get_one(&conn, &product_example.code).unwrap();
        assert_eq!(product_example, product_from_db);

        // Get all.
        let products = Product::get_all(&conn);
        assert!(products.len() > 1);

        ///////////////////////////////////////////////////////////////////////
        // PRODUCT HISTORY
        ///////////////////////////////////////////////////////////////////////
        // Remove all.
        Product::remove_all_history(&conn);

        // Insert.
        let mut product_example = product_example!(now);
        // To exemplify transaction. 
        let tx = conn.transaction().unwrap();
        product_example.save_history(&tx);
        product_example.changed_at = now + chrono::Duration::seconds(1);
        product_example.save_history(&tx);
        tx.commit().unwrap();

        assert_eq!(Product::get_all(&conn).len(), 2);
    }
}
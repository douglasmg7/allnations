use chrono::{DateTime, FixedOffset, Utc, SecondsFormat};
// #[macro_use]
use lazy_static;
use std::fmt;

const PRODUCT_FIELDS: &str =
    "zunka_product_id, code, description, timestamp, department, category, sub_category, maker, \
technical_description, url_image, part_number, ean, ncm, price_sale, price_without_st, \
icms_st_taxation, warranty_month, length_mm, width_mm, height_mm, weight_g, active, \
availability, origin, stock_origin, stock_qtd, created_at, changed_at, removed_at, checked_at";

#[allow(unused_macros)]
macro_rules! product_example {
    ($now: ident) => {
        Product{
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
            checked_at: ZERO_TIME.clone(),
            removed_at: ZERO_TIME.clone(),
        };
    }
}

// Create execute named product for rusqlite statement.
#[allow(unused_macros)]
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
        Product {
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

// const ZERO_TIME: &str = "0001-01-01T03:00:00-03:00";
lazy_static::lazy_static! {
    pub static ref ZERO_TIME: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("0001-01-01T03:00:00-03:00").unwrap();

    // Insert product.
    static ref  PRODUCT_FIELDS_INSERT: String = PRODUCT_FIELDS
        .split(",")
        .map(|x| format!(":{0}", x.trim()))
        .collect::<Vec<_>>()
        .join(", ");
    static ref SQL_INSERT: String = format!(
        r#"INSERT INTO product ({}) VALUES ({})"#,
        PRODUCT_FIELDS, &*PRODUCT_FIELDS_INSERT // &* To go inside the wrap.
    );
}

// Remove all.
pub fn remove_all(conn: &rusqlite::Connection) {
    conn.execute("DELETE FROM product", rusqlite::NO_PARAMS)
        .unwrap();
}

// Get all.
pub fn get_all(conn: &rusqlite::Connection) -> Option<Vec<Product>> {
    let mut stmt = conn
        .prepare(&format!("SELECT {} FROM product", PRODUCT_FIELDS))
        .unwrap();
    let iter = stmt
        .query_map(rusqlite::params![], |row| Ok(product_from_row!(row)))
        .unwrap();
    let mut products = Vec::new();
    for product in iter {
        products.push(product.unwrap());
    }
    Some(products)
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
    pub price_sale: i32,
    pub price_without_st: i32,
    pub icms_st_taxation: bool,
    pub warranty_month: i32,
    pub length_mm: i32,
    pub width_mm: i32,
    pub height_mm: i32,
    pub weight_g: i32,
    pub active: bool,
    pub availability: bool,
    pub origin: String,     // Nacional, importado, entre outros...
    pub stock_origin: String,   // RJ, SC or ES.
    pub stock_qtd: i32,
    pub created_at: DateTime<FixedOffset>,
    pub changed_at: DateTime<FixedOffset>,
    pub checked_at: DateTime<FixedOffset>,
    pub removed_at: DateTime<FixedOffset>,
}

impl Product {
    pub fn new() -> Self {
        let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600));
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

    // Save.
    pub fn save(&self, conn: &rusqlite::Connection) {
        let mut stmt = conn.prepare(&SQL_INSERT).unwrap();
        stmt_execute_named_product!(stmt, self);
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

// Read xml from stdin.
pub fn products_from_xml<T: std::io::Read>(xml: T) -> Vec<Product> {
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
                        .parse::<i32>()
                        .expect(&format!("Invalid GARANTIA: {}", text));
                }
                "PESOKG" => {
                    product.weight_g = (1000.0
                        * text
                            .parse::<f32>()
                            .expect(&format!("Invalid PESOKG: {}", text)))
                        as i32;
                }
                "PRECOREVENDA" => {
                    product.price_sale = (100.0
                        * text
                            .parse::<f32>()
                            .expect(&format!("Invalid PRECOREVENDA: {}", text)))
                        as i32;
                }
                "PRECOSEMST" => {
                    product.price_without_st = (100.0
                        * text
                            .parse::<f32>()
                            .expect(&format!("Invalid PRECOSEMST: {}", text)))
                        as i32;
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
                        as i32;
                }
                "ALTURA" => {
                    product.height_mm = (1000.0
                        * text
                            .parse::<f32>()
                            .expect(&format!("Invalid ALTURA: {}", text)))
                        as i32;
                }
                "PROFUNDIDADE" => {
                    product.length_mm = (1000.0
                        * text
                            .parse::<f32>()
                            .expect(&format!("Invalid PROFUNDIDADE: {}", text)))
                        as i32;
                }
                "ATIVO" => product.active = text == "1",
                "SUBSTTRIBUTARIA" => product.icms_st_taxation = text == "1",
                "ORIGEMPRODUTO" => product.origin = text.clone(),
                "ESTOQUEDISPONIVEL" => {
                    product.stock_qtd = (text
                        .parse::<f32>()
                        .expect(&format!("Invalid ESTOQUEDISPONIVEL: {}", text)))
                        as i32
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
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    products
}

#[allow(unused_imports)]
mod test {
    use super::super::config::Config;
    use super::super::db::Db;
    use super::*;

    #[test]
    fn crud() {
        let conn = Db::new(&Config::new().db_filename).conn;

        // Remove all.
        remove_all(&conn);

        // Insert.
        let now = Utc::now().with_timezone(&FixedOffset::west(3 * 3600));
        // let mut product_insert = product_example!(now);
        product_example!(now).save(&conn);
        // product_insert.save(&conn);
        Product::new().save(&conn);

        // Get all.
        let products = get_all(&conn).unwrap();
        assert!(products.len() > 1);
    }
}
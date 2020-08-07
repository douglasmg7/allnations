use chrono::{DateTime, FixedOffset, Utc, SecondsFormat};
// #[macro_use]
use lazy_static;
use std::fmt;

// const ZERO_TIME: &str = "0001-01-01T03:00:00-03:00";
lazy_static::lazy_static! {
   pub static ref ZERO_TIME: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("0001-01-01T03:00:00-03:00").unwrap();
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
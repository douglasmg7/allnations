use once_cell::sync::OnceCell;
use std::env;
use std::io::{self};
use xml::reader::{EventReader, XmlEvent};

// Production mode.
static PRODUCTION: OnceCell<bool> = OnceCell::new();
static DEFAULT_TIMESTAMP: &str = "0000-01-01T00:00:00-00:00";

impl Product {
    fn new() -> Self {
        Product {
            // timestamp: chrono::FixedOffset::west(3),
            // timestamp: chrono::Utc::now(),
            timestamp: chrono::DateTime::parse_from_rfc3339(DEFAULT_TIMESTAMP)
                .expect(&format!("Error parsing: {}", DEFAULT_TIMESTAMP)),
            department: String::new(),
            category: String::new(),
            sub_category: String::new(),
            maker: String::new(),
            code: String::new(),
            description: String::new(),
            description_tec: String::new(),
            part_number: String::new(),
            ean: String::new(),
            warranty_month: 0,
            weight_g: 0,
            price_sale: 0,
            price_without_st: 0,
            availability: false,
            url_image: String::new(),
            // RJ, SC or ES.
            stock_origin: String::new(),
            ncm: String::new(),
            width_mm: 0,
            height_mm: 0,
            depth_mm: 0,
            active: false,
            icms_st_taxation: false,
            // Nacional, importado, entre outros...
            origin: String::new(),
            stock_qtd: 0,
        }
    }
}

// Aldo product.
#[derive(Debug)]
struct Product {
    timestamp: chrono::DateTime<chrono::offset::FixedOffset>,
    department: String,
    category: String,
    sub_category: String,
    maker: String,
    code: String,
    description: String,
    description_tec: String,
    part_number: String,
    ean: String,
    warranty_month: i32,
    weight_g: i32,
    price_sale: i32,
    price_without_st: i32,
    availability: bool,
    url_image: String,
    // RJ, SC or ES.
    stock_origin: String,
    ncm: String,
    width_mm: i32,
    height_mm: i32,
    depth_mm: i32,
    active: bool,
    icms_st_taxation: bool,
    // Nacional, importado, entre outros...
    origin: String,
    stock_qtd: i32,
}

// Set run mode.
pub fn set_run_mode() {
    // Set run mode.
    match env::var_os("RUN_MODE") {
        Some(val) => {
            if val == "production" {
                PRODUCTION.set(true).unwrap();
            } else {
                PRODUCTION.set(false).unwrap();
            }
        }
        None => {
            PRODUCTION.set(false).unwrap();
        }
    }
    // Print run mode and version.
    println!(
        "Runing in {} mode (version {})",
        if is_production() {
            "production"
        } else {
            "development"
        },
        env!("CARGO_PKG_VERSION")
    );
}

// If in production mode.
pub fn is_production() -> bool {
    if *PRODUCTION.get().expect("Run mode not defined") {
        return true;
    } else {
        return false;
    }
}

// Read xml from stdin.
pub fn read_stdin() -> io::Result<()> {
    // let mut buffer = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let parser = EventReader::new(handle);
    // let mut depth = 0;
    let mut tag = String::new();
    let mut inside_product = false;
    let mut products: Vec<Product> = Vec::new();
    let mut product = Product::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
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
            Ok(XmlEvent::Characters(text)) => match tag.as_str() {
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
                "DESCRTEC" => product.description_tec = text.clone(),
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
                    product.depth_mm = (1000.0
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
            Ok(XmlEvent::EndElement { name }) => {
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
    // println!("Products: {:?}", products);
    for i in 0..3 {
        println!("Cound: {}", i);
        println!(
            "Product: {} - {}",
            products[i].code, products[i].description
        );
    }
    println!("-Products quantity: {}", products.len());

    // handle.read_to_string(&mut buffer)?;
    // println!("stdin: {}", buffer);
    Ok(())
}
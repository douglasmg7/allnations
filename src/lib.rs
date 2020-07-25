use once_cell::sync::OnceCell;
use std::env;
use std::fmt;
use std::io::{self};
use xml::reader::{EventReader, XmlEvent};

// Production mode.
static PRODUCTION: OnceCell<bool> = OnceCell::new();
static DEFAULT_TIMESTAMP: &str = "0000-01-01T00:00:00-00:00";

// Aldo product.
#[derive(Debug)]
struct Product {
    code: String,
    description: String,
    timestamp: chrono::DateTime<chrono::offset::FixedOffset>,
    department: String,
    category: String,
    sub_category: String,
    maker: String,
    description_tec: String,
    url_image: String,
    part_number: String,
    ean: String,
    ncm: String,
    price_sale: i32,
    price_without_st: i32,
    icms_st_taxation: bool,
    warranty_month: i32,
    width_mm: i32,
    height_mm: i32,
    depth_mm: i32,
    weight_g: i32,
    active: bool,
    availability: bool,
    origin: String,     // Nacional, importado, entre outros...
    stock_origin: String,   // RJ, SC or ES.
    stock_qtd: i32,
}

impl Product {
    fn new() -> Self {
        Product {
            code: String::new(),
            description: String::new(),
            timestamp: chrono::DateTime::parse_from_rfc3339(DEFAULT_TIMESTAMP)
                .expect(&format!("Error parsing: {}", DEFAULT_TIMESTAMP)),
            department: String::new(),
            category: String::new(),
            sub_category: String::new(),
            maker: String::new(),
            description_tec: String::new(),
            url_image: String::new(),
            part_number: String::new(),
            ean: String::new(),
            ncm: String::new(),
            price_sale: 0,
            price_without_st: 0,
            icms_st_taxation: false,
            warranty_month: 0,
            width_mm: 0,
            height_mm: 0,
            depth_mm: 0,
            weight_g: 0,
            active: false,
            availability: false,
            origin: String::new(),
            stock_origin: String::new(),
            stock_qtd: 0,
        }
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[product]\n\tcode: {}\n\tdescription: {}\n\ttimestamp: {}\n\tdepartment: {}\n\tcategory: {}\n\tsub_category: {}\n\tmaker: {}\n\tdescription_tec bytes count: {}\
            \n\turl_image: {}\n\tpart_number: {}\n\tean: {}\n\tncm: {}\
            \n\tprice_sale: {}\n\tprice_without_st: {}\n\ticms_st_taxation: {}\n\twarranty_month: {}\
            \n\twidth_mm: {}\n\theight_mm: {}\n\tdepth_mm: {}\n\tweight_g: {}\
            \n\tactive: {}\n\tavailability: {}\n\torigin: {}\n\tstock_origin: {}\n\tstock_qtd: {}",
            self.code, self.description, self.timestamp, self.department, self.category, self.sub_category, self.maker, self.description_tec.len(), 
            self.url_image, self.part_number, self.ean, self.ncm,
            self.price_sale, self.price_without_st, self.icms_st_taxation, self.warranty_month,
            self.width_mm, self.height_mm, self.depth_mm, self.weight_g,
            self.active, self.availability, self.origin, self.stock_origin, self.stock_qtd 
        )
    }
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
        println!("{}", products[i]);
        // println!("Cound: {}", i);
        // println!(
        // "Product: {} - {}",
        // products[i].code, products[i].description
        // );
    }
    println!("-Products quantity: {}", products.len());

    // handle.read_to_string(&mut buffer)?;
    // println!("stdin: {}", buffer);
    Ok(())
}
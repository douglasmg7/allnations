use once_cell::sync::OnceCell;
// use serde::{Deserialize, Serialize};
// use serde_xml_rs::{from_str, to_string};
// use serde_xml_rs;
use std::env;
use std::io::{self, Read};

use xml::reader::{EventReader, XmlEvent};

// #[macro_use]
// extern crate serde_derive;

static PRODUCTION: OnceCell<bool> = OnceCell::new();

#[derive(Debug, Default)]
struct Product {
    timestamp: String,
    department: String,
    category: String,
    sub_category: String,
    maker: String,
    code: String,
    description: String,
    description_tec: String,
    part_number: String,
    ean: String,
    warranty: String,
    weight_kg: f64,
    price_sale: f64,
    price_no_st: f64,
    availability: bool,
    url_image: String,
    stock: i32,
    ncm: String,
    width_mm: i32,
    height_mm: i32,
    deep_mm: i32,
    active: bool,
    tax_substitute: String,
    origin: String,
    available_stock: bool,
}

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

pub fn is_production() -> bool {
    if *PRODUCTION.get().expect("Run mode not defined") {
        return true;
    } else {
        return false;
    }
}

pub fn read_stdin() -> io::Result<()> {
    // let mut buffer = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    let parser = EventReader::new(handle);
    // let mut depth = 0;
    let mut tag = String::new();
    let mut inside_product = false;
    let mut products: Vec<Product> = Vec::new();
    let mut product = Product::default();
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
                "TIMESTAMP" => product.timestamp = text.clone(),
                "DEPARTAMENTO" => product.department = text.clone(),
                "CATEGORIA" => product.category = text.clone(),
                "SUBCATEGORIA" => product.sub_category = text.clone(),
                "FABRICANTE" => product.maker = text.clone(),
                "CODIGO" => product.code = text.clone(),
                "DESCRICAO" => product.description = text.clone(),
                "DESCRTEC" => product.description_tec = text.clone(),
                "PARTNUMBER" => product.part_number = text.clone(),
                "EAN" => product.ean = text.clone(),
                "GARANTIA" => product.warranty = text.clone(),
                "PESOKG" => product.weight_kg = text.clone(),
                "PRECOREVENDA" => product.price_sale = text.clone(),
                "PRECOSEMST" => product.price_no_st = text.clone(),
                "DISPONIVEL" => product.availability = text.clone(),
                "URLFOTOPRODUTO" => product.url_image = text.clone(),
                "ESTOQUE" => product.stock = text.clone(),
                "NCM" => product.ncm = text.clone(),
                "LARGURA" => product.width_mm = text.clone(),
                "ALTURA" => product.height_mm = text.clone(),
                "PROFUNDIDADE" => product.deep_mm = text.clone(),
                "ATIVO" => product.active = text.clone(),
                "SUBSTTRIBUTARIA" => product.tax_substitute = text.clone(),
                "ORIGEMPRODUTO" => product.origin = text.clone(),
                "ESTOQUEDISPONIVEL" => product.available_stock = text.clone(),
                _ => {}
            },
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name.eq("Produtos") {
                    products.push(product);
                    product = Product::default();
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
/*****************************************************
* PRODUCT
*****************************************************/
// Product example.
#[macro_export]
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
            stock_qty: 42,
            created_at: $now.clone(),
            changed_at: $now.clone(),
            checked_at: ZERO_TIME.clone(),
            removed_at: ZERO_TIME.clone(),
        };
    }
}

// Create execute named product for rusqlite statement.
#[macro_export]
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
                (":stock_qty", &$product.stock_qty),
                (":created_at", &$product.created_at),
                (":changed_at", &$product.changed_at),
                (":checked_at", &$product.checked_at),
                (":removed_at", &$product.removed_at),
            ])
            .unwrap();
    };
}

#[macro_export]
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
            stock_qty: $row.get(25).unwrap(),
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

/*****************************************************
* CATEGORY
*****************************************************/
// Create execute named category for rusqlite statement.
#[macro_export]
macro_rules! stmt_execute_named_category {
    ($stmt: ident, $category: ident) => {
        $stmt
            .execute_named(&[
                (":name", &$category.name),
                (":text", &$category.text),
                (":products_qty", &$category.products_qty),
                (":selected", &$category.selected),
            ])
            .unwrap();
    };
}

// Create category from a row.
#[macro_export]
macro_rules! category_from_row {
    ($row: ident) => {
        Category {
            name: $row.get(0).unwrap(),
            text: $row.get(1).unwrap(),
            products_qty: $row.get(2).unwrap(),
            selected: $row.get(3).unwrap(),
        }
    };
}

///////////////////////////////////////////////////////////////////////////////
// UTIL
///////////////////////////////////////////////////////////////////////////////
#[macro_export]
macro_rules! now {
    () => {
        Utc::now().with_timezone(&FixedOffset::west(3 * 3600));
    };
}

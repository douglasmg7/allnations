use std::fmt;

// Delete all.
pub fn delete_all(conn: &rusqlite::Connection) {
    conn.execute("DELETE FROM category", rusqlite::NO_PARAMS)
        .unwrap();
}

#[derive(Debug)]
pub struct Category {
    pub name: String, // Text without space.
    pub text: String,
    pub products_qtd: i32,
    pub selected: bool,
}

impl Category {
    pub fn new(text: &str, quantity: i32, selected: bool) -> Category {
        let text: Vec<_> = text.split_whitespace().collect();
        Category {
            name: text.join("_").to_uppercase(),
            text: text.join(" "),
            products_qtd: quantity,
            selected: selected,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[category]\n\tname: {}\n\ttext: {}\n\tproduct_qtd: {}\n\tselected: {}",
            self.name, self.text, self.products_qtd, self.selected,
        )
    }
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn create_category() {
        let cat = Category::new(" SuPER   CATEgory a   ", 32, true);
        assert_eq!(cat.name, "SUPER_CATEGORY_A");
        assert_eq!(cat.text, "SuPER CATEgory a");
    }

    // #[test]
    // fn crud() {
    // // Configuration.
    // let config = config::Config::new();
    // let db = db::Db::new(&config.db_filename);

    // delete_all(db.conn);
    // }
}
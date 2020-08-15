use std::fmt;

// Create execute named category for rusqlite statement.
macro_rules! stmt_execute_named_category {
    ($stmt: ident, $category: ident) => {
        $stmt
            .execute_named(&[
                (":name", &$category.name),
                (":text", &$category.text),
                (":products_qtd", &$category.products_qtd),
                (":selected", &$category.selected),
            ])
            .unwrap();
    };
}

// Create category from a row.
macro_rules! category_from_row {
    ($row: ident) => {
        Category {
            name: $row.get(0).unwrap(),
            text: $row.get(1).unwrap(),
            products_qtd: $row.get(2).unwrap(),
            selected: $row.get(3).unwrap(),
        }
    };
}

// Remove all.
pub fn remove_all(conn: &rusqlite::Connection) {
    conn.execute("DELETE FROM category", rusqlite::NO_PARAMS)
        .unwrap();
}

// Get all.
pub fn get_all(conn: &rusqlite::Connection) -> Option<Vec<Category>> {
    let mut stmt = conn
        .prepare("SELECT name, text, products_qtd, selected FROM category")
        .unwrap();
    let categories_iter = stmt
        .query_map(rusqlite::params![], |row| Ok(category_from_row!(row)))
        .unwrap();
    let mut categories = Vec::new();
    for category in categories_iter {
        categories.push(category.unwrap());
    }
    Some(categories)
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

    // Insert on db.
    pub fn save(&self, conn: &rusqlite::Connection) {
        let sql_insert_category = "INSERT INTO category (name, text, products_qtd, selected) values (:name, :text, :products_qtd, :selected)";
        let mut stmt = conn.prepare(sql_insert_category).unwrap();
        stmt_execute_named_category!(stmt, self);
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

#[allow(unused_imports)]
mod test {
    use super::super::config::Config;
    use super::super::db::Db;
    use super::*;

    #[test]
    fn new() {
        let cat = super::Category::new(" SuPER   CATEgory a   ", 32, true);
        assert_eq!(cat.name, "SUPER_CATEGORY_A");
        assert_eq!(cat.text, "SuPER CATEgory a");
    }

    #[test]
    fn crud() {
        let conn = Db::new(&Config::new().db_filename).conn;

        // Remove all.
        remove_all(&conn);

        // Insert.
        Category::new("Laptops", 2, true).save(&conn);
        Category::new("Computadores", 4, true).save(&conn);

        // Get all.
        let categories = get_all(&conn).unwrap();
        assert!(categories.len() > 1);
    }
}
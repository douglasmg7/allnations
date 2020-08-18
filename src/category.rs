use std::fmt;

#[derive(Debug, PartialEq)]
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

    // Create category name from category text.
    pub fn name_from_text(text: &str) -> String {
        text.split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
            .to_uppercase()
    }

    // Insert on db.
    pub fn save(&self, conn: &rusqlite::Connection) {
        let sql = "INSERT INTO category (name, text, products_qtd, selected) VALUES (:name, :text, :products_qtd, :selected)";
        let mut stmt = conn.prepare(sql).unwrap();
        super::stmt_execute_named_category!(stmt, self);
    }

    // Insert or update on db.
    pub fn save_or_update_only_products_qtd(&self, conn: &rusqlite::Connection) {
        let sql = "INSERT INTO category (name, text, products_qtd, selected) VALUES (:name, :text, :products_qtd, :selected)\
                   ON CONFLICT(name) DO UPDATE SET products_qtd=excluded.products_qtd";
        let mut stmt = conn.prepare(sql).unwrap();
        super::stmt_execute_named_category!(stmt, self);
    }

    // Update on db.
    pub fn update(&self, conn: &rusqlite::Connection) {
        let sql = r#"UPDATE category SET text = :text, products_qtd = :products_qtd, selected = :selected WHERE name = :name"#;
        let mut stmt = conn.prepare(sql).unwrap();
        super::stmt_execute_named_category!(stmt, self);
    }

    // Remove all.
    pub fn remove_all(conn: &rusqlite::Connection) {
        conn.execute("DELETE FROM category", rusqlite::NO_PARAMS)
            .unwrap();
    }

    // Get one from db.
    pub fn get_one(conn: &rusqlite::Connection, name: &str) -> Option<Category> {
        let mut stmt = conn
            .prepare("SELECT name, text, products_qtd, selected FROM category WHERE name = :name")
            .unwrap();
        let mut rows = stmt.query_named(&[(":name", &name)]).unwrap();

        let row = rows.next().unwrap();
        match row {
            None => None,
            Some(row) => Some(super::category_from_row!(row)),
        }
    }

    // Get all.
    pub fn get_all(conn: &rusqlite::Connection) -> Option<Vec<Category>> {
        let mut stmt = conn
            .prepare("SELECT name, text, products_qtd, selected FROM category")
            .unwrap();
        let categories_iter = stmt
            .query_map(rusqlite::params![], |row| {
                Ok(super::category_from_row!(row))
            })
            .unwrap();
        let mut categories = Vec::new();
        for category in categories_iter {
            categories.push(category.unwrap());
        }
        Some(categories)
    }

    // Get all selected.
    pub fn get_all_selected(conn: &rusqlite::Connection) -> Vec<Category> {
        let mut stmt = conn
            .prepare(
                "SELECT name, text, products_qtd, selected FROM category WHERE selected = true",
            )
            .unwrap();
        let categories_iter = stmt
            .query_map(rusqlite::params![], |row| {
                Ok(super::category_from_row!(row))
            })
            .unwrap();
        let mut categories = Vec::new();
        for category in categories_iter {
            categories.push(category.unwrap());
        }
        categories
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
    use super::*;

    #[test]
    fn new() {
        let cat = super::Category::new(" SuPER   CATEgory a   ", 32, true);
        assert_eq!(cat.name, "SUPER_CATEGORY_A");
        assert_eq!(cat.text, "SuPER CATEgory a");
    }

    #[test]
    fn crud() {
        let conn = rusqlite::Connection::open(&Config::new().db_filename).unwrap();

        // Remove all.
        Category::remove_all(&conn);

        // Insert.
        Category::new("Laptops", 2, true).save(&conn);
        Category::new("Computadores", 4, true).save(&conn);

        // Get all.
        let categories = Category::get_all(&conn).unwrap();
        assert!(categories.len() > 1);

        // Insert or update.
        // Must insert.
        let mut category = Category::new("Impressoras", 5, true);
        category.save_or_update_only_products_qtd(&conn);
        let saved_category = Category::get_one(&conn, &category.name).unwrap();
        assert_eq!(saved_category, category);
        // Must update.
        category.products_qtd = 10;
        category.selected = false;
        category.save_or_update_only_products_qtd(&conn);
        let saved_category = Category::get_one(&conn, &category.name).unwrap();
        assert_eq!(saved_category.text, category.text);
        assert_eq!(saved_category.products_qtd, category.products_qtd);
        // Must not update selected.
        assert_eq!(saved_category.selected, true);

        // Update.
        let mut category = Category::new("hds", 20, false);
        category.save(&conn);
        category.products_qtd = 10;
        category.selected = true;
        category.update(&conn);
        let saved_category = Category::get_one(&conn, &category.name).unwrap();
        assert_eq!(saved_category, category);
    }
}
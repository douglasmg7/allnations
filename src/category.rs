use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Category {
    pub name: String,
    pub products_qty: i32,
    pub selected: bool,
}

impl Category {
    pub fn new(name: &str, quantity: i32, selected: bool) -> Category {
        // let text: Vec<_> = text.split_whitespace().collect();
        Category {
            name: name.to_string(),
            // name: text.join("_").to_lowercase(),
            // text: text.join(" ").to_lowercase(),
            products_qty: quantity,
            selected: selected,
        }
    }

    // Sanitizer name.
    pub fn sanitizer_name(text: &str) -> String {
        text.split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
            .to_uppercase()
    }

    // Insert on db.
    pub fn save(&self, conn: &rusqlite::Connection) {
        let sql = "INSERT INTO category (name, products_qty, selected) VALUES (:name, :products_qty, :selected)";
        let mut stmt = conn.prepare(sql).unwrap();
        super::stmt_execute_named_category!(stmt, self);
    }

    // Insert or update on db.
    pub fn save_or_update_only_products_qty(&self, conn: &rusqlite::Connection) {
        let sql = "INSERT INTO category (name, products_qty, selected) VALUES (:name, :products_qty, :selected)\
                   ON CONFLICT(name) DO UPDATE SET products_qty=excluded.products_qty";
        let mut stmt = conn.prepare(sql).unwrap();
        super::stmt_execute_named_category!(stmt, self);
    }

    // Update on db.
    pub fn update(&self, conn: &rusqlite::Connection) {
        let sql = r#"UPDATE category SET products_qty = :products_qty, selected = :selected WHERE name = :name"#;
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
            .prepare("SELECT name, products_qty, selected FROM category WHERE name = :name")
            .unwrap();
        let mut rows = stmt.query_named(&[(":name", &name)]).unwrap();

        let row = rows.next().unwrap();
        match row {
            None => None,
            Some(row) => Some(super::category_from_row!(row)),
        }
    }

    // Get all.
    pub fn get_all(conn: &rusqlite::Connection) -> Vec<Category> {
        let mut stmt = conn
            .prepare("SELECT name, products_qty, selected FROM category")
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

    // Get all selected.
    pub fn get_all_selected(conn: &rusqlite::Connection) -> Vec<Category> {
        let mut stmt = conn
            .prepare("SELECT name, products_qty, selected FROM category WHERE selected = true")
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
            "[category]\n\tname: {}\n\tproduct_qty: {}\n\tselected: {}",
            self.name, self.products_qty, self.selected,
        )
    }
}

#[allow(unused_imports)]
mod test {
    use super::super::config::Config;
    use super::*;

    #[test]
    fn new() {
        let cat = super::Category::new(
            &super::Category::sanitizer_name(" SuPER   CATEgory a   "),
            32,
            true,
        );
        assert_eq!(cat.name, "SUPER CATEGORY A");
    }

    #[test]
    fn crud() {
        let conn = rusqlite::Connection::open(&Config::new().db_filename).unwrap();

        // Remove all.
        Category::remove_all(&conn);

        // Insert.
        Category::new("LAPTOPS", 2, true).save(&conn);
        Category::new("COMPUTADORES", 4, true).save(&conn);

        // Get all.
        let categories = Category::get_all(&conn);
        assert!(categories.len() > 1);

        // Insert or update.
        // Must insert.
        let mut category = Category::new("IMPRESSORAS", 5, true);
        category.save_or_update_only_products_qty(&conn);
        let saved_category = Category::get_one(&conn, &category.name).unwrap();
        assert_eq!(saved_category, category);
        // Must update.
        category.products_qty = 10;
        category.selected = false;
        category.save_or_update_only_products_qty(&conn);
        let saved_category = Category::get_one(&conn, &category.name).unwrap();
        assert_eq!(saved_category.products_qty, category.products_qty);
        // Must not update selected.
        assert_eq!(saved_category.selected, true);

        // Update.
        let mut category = Category::new("HDS", 20, false);
        category.save(&conn);
        category.products_qty = 10;
        category.selected = true;
        category.update(&conn);
        let saved_category = Category::get_one(&conn, &category.name).unwrap();
        assert_eq!(saved_category, category);
    }
}
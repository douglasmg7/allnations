use std::fmt;

#[derive(Debug)]
pub struct Category {
    pub name: String, // Text without space.
    pub text: String,
    pub products_qtd: String,
    pub selected: bool,
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
use yansi::{Color, Paint};

pub trait Paintable {
    fn paint(&self, color: Color) -> String;

    fn error(&self) -> String {
        self.paint(Color::Red)
    }
}

impl Paintable for &str {
    fn paint(&self, color: Color) -> String {
        Paint::new(self).fg(color).to_string()
    }
}

impl Paintable for String {
    fn paint(&self, color: Color) -> String {
        Paint::new(self).fg(color).to_string()
    }
}

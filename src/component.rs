use crate::style::Style;

pub struct Component {
    style: Style,
    text: String,
}

impl Component {
    pub fn text(text: String) -> Self {
        Self {
            style: Style::default(),
            text,
        }
    }
    pub fn render(&self) -> String {
        self.style.render(&self.text)
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

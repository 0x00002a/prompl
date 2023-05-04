use crate::style::Style;

#[derive(Clone, Debug)]
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
    pub fn append(mut self, txt: &str) -> Self {
        self.text += txt;
        self
    }
    pub fn render(&self) -> String {
        self.style.render(&self.text)
    }
    pub fn get_style(&self) -> &Style {
        &self.style
    }
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

use crate::colours::Colour;

#[derive(Clone, Debug)]
pub struct Style {
    bold: bool,
    italic: bool,
    fg: Option<Colour>,
    bg: Option<Colour>,
}
fn escape(txt: &str) -> String {
    let mut s = "%{".to_owned();
    s.push_str(txt);
    s.push_str("%}");
    s
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_bg(&self) -> Option<&Colour> {
        self.bg.as_ref()
    }
    pub fn render(&self, wrap: &str) -> String {
        let mut s = wrap.to_owned();
        if self.bold {
            s = format!("%B{s}%b")
        }
        if self.italic {
            s = escape(r"\e[3m") + s.as_str() + escape(r"\e[0m").as_str();
        }
        if let Some(fg) = self.fg {
            let mut sv = "%F{".to_owned();
            sv += &fg.code().to_string();
            sv.push_str("}");
            sv += &s;
            sv.push_str("%f");
            s = sv;
        }
        if let Some(bg) = self.bg {
            let mut sv = "%K{".to_owned();
            sv += &bg.code().to_string();
            sv.push_str("}");
            sv += &s;
            sv.push_str("%k");
            s = sv;
        }
        s
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
    pub fn fg(mut self, fg: Colour) -> Self {
        self.fg.replace(fg);
        self
    }
    pub fn no_bg(mut self) -> Self {
        self.bg.take();
        self
    }
    pub fn bg(mut self, bg: Colour) -> Self {
        self.bg.replace(bg);
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            fg: Default::default(),
            bg: Default::default(),
        }
    }
}

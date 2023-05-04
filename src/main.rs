use colours::NonPrompt;
use component::Component;
use style::Style;

mod colours;
mod component;
mod style;

fn path_comp() -> Component {
    let path_sel = "[[%~]]";
    Component::text(path_sel.to_owned())
        .style(Style::new().fg(NonPrompt::Gray50).bg(NonPrompt::Gray100))
}

fn components() -> Vec<Component> {
    vec![path_comp()]
}
fn join_comps(comps: &[Component], sep: &str) -> String {
    comps
        .iter()
        .map(|c| c.render())
        .collect::<Vec<_>>()
        .join(sep)
        + sep
}
const POWERLINE_SEP: &str = "";

fn main() {
    print!("{}", join_comps(&components(), POWERLINE_SEP))
}

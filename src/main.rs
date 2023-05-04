use colours::Colour;
use component::Component;
use rand::{seq::SliceRandom, thread_rng};
use style::Style;

mod colours;
mod component;
mod style;

fn path_comp() -> Component {
    let path_sel = "%~";
    Component::text(path_sel.to_owned())
        .style(Style::new().fg(Colour::Gray100).bg(Colour::Gray50).bold())
}

fn components() -> Vec<Component> {
    vec![path_comp()]
}
fn join_comps(comps: &[Component], sep: &str) -> String {
    comps.iter().fold(String::new(), |s, c| {
        let r = c.clone().render();
        let mut sep_style = c.get_style().clone().no_bg();
        if let Some(bg) = c.get_style().get_bg() {
            sep_style = sep_style.fg(*bg);
        }
        let sep = Component::text(sep.to_owned()).style(sep_style);
        s + &r + &sep.render()
    })
}
const POWERLINE_SEP: &str = "";
const PROMPT_CHARS: [char; 11] = ['€', '#', '$', 'λ', '!', ':', '?', '\\', '/', '«', '*'];
const PROMPT_COLOURS: [Colour; 6] = [
    Colour::SeaGreen,
    Colour::MintyRose,
    Colour::Plum,
    Colour::SteelBlue,
    Colour::RosyBrown,
    Colour::Silver,
];

fn prompt_comp(chars: &[char], colours: &[Colour]) -> Component {
    assert!(!chars.is_empty());
    assert!(!colours.is_empty());

    let prefix = chars.choose(&mut thread_rng()).unwrap();
    let colour = colours.choose(&mut thread_rng()).unwrap();
    Component::text(format!("{}> ", prefix)).style(Style::new().fg(*colour).bold())
}

fn main() {
    let status = join_comps(&components(), POWERLINE_SEP);
    let prompt = prompt_comp(&PROMPT_CHARS, &PROMPT_COLOURS).render();
    print!("{}\n{}", status, prompt)
}

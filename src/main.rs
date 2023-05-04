use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

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
fn git_comp() -> Option<Component> {
    let cmd = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .stderr(Stdio::null())
        .output()
        .ok()?;
    if !cmd.status.success() || cmd.stdout.is_empty() {
        return None;
    }
    let mut root = cmd.stdout;
    if root[root.len() - 1] as char == '\n' {
        root.pop();
    }
    let root = PathBuf::from(String::from_utf8(root).ok()?);
    let mut head = File::open(root.join(".git").join("HEAD")).ok()?;
    let mut headref = String::new();
    head.read_to_string(&mut headref).ok()?;
    let mut branch = headref.split("/").last()?.to_owned();
    branch.pop();
    Some(
        Component::text(format!("\u{e0a0} {}", branch))
            .style(Style::new().bg(Colour::SkyBlue1).fg(Colour::Gray3).bold()),
    )
}

fn components() -> Vec<Component> {
    let mut comps = vec![path_comp()];
    if let Some(c) = git_comp() {
        comps.push(c);
    }
    comps
}
fn join_comps(comps: &[Component], sep: &str) -> String {
    let mut out = Vec::new();
    for n in 0..comps.len() {
        let c = &comps[n];
        let r = c.clone().append(" ").render();
        let mut sep_style = c.get_style().clone().no_bg();
        if let Some(bg) = c.get_style().get_bg() {
            sep_style = sep_style.fg(*bg);
        }
        if let Some(bg) = comps.get(n + 1).and_then(|c| c.get_style().get_bg()) {
            sep_style = sep_style.bg(*bg);
        }
        let sep = Component::text(format!("{} ", sep)).style(sep_style);
        out.push(r);
        out.push(sep.render());
    }
    out.join("")
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

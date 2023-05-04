use std::{
    fs::File,
    io::Read,
    path::PathBuf,
    process::{Command, Stdio},
};

use colours::{Basic, Colour, C256};
use component::Component;
use rand::{seq::SliceRandom, thread_rng};
use style::Style;

mod colours;
mod component;
mod style;

fn path_comp() -> Component {
    let path_sel = "%~";
    Component::text(path_sel.to_owned())
        .style(Style::new().fg(C256::Gray100).bg(C256::Gray50).bold())
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
            .style(Style::new().bg(C256::SkyBlue1).fg(C256::Gray3).bold()),
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
    Colour::C256(C256::SeaGreen),
    Colour::C256(C256::MintyRose),
    Colour::C256(C256::Plum),
    Colour::C256(C256::SteelBlue),
    Colour::C256(C256::RosyBrown),
    Colour::C256(C256::Silver),
];

fn prompt_comp(chars: &[char], colours: &[Colour]) -> Component {
    assert!(!chars.is_empty());
    assert!(!colours.is_empty());

    let prefix = chars.choose(&mut thread_rng()).unwrap();
    let colour = colours.choose(&mut thread_rng()).unwrap();
    let prefix_style = Style::new().fg(*colour);
    let prefix = Component::text(prefix.to_string()).style(prefix_style.clone());
    let suffix_base = Component::text(">".to_owned());
    let suffix = exit_code_switch(
        &suffix_base.clone().style(prefix_style.clone()),
        &suffix_base.style(prefix_style.fg(Basic::Red)),
    );
    Component::text(format!("{}{} ", prefix.render(), suffix.render())).style(Style::new().bold())
}
fn exit_code_switch(success: &Component, failure: &Component) -> Component {
    Component::text(format!("%(?.{0}.{1})", success.render(), failure.render()))
}

fn gen_prompt() -> String {
    let status = join_comps(&components(), POWERLINE_SEP);
    let prompt = prompt_comp(&PROMPT_CHARS, &PROMPT_COLOURS).render();
    status + "\n" + &prompt
}

fn main() {
    std::panic::set_hook(Box::new(|i| {
        let v = i.payload();
        let msg = if let Some(v) = v.downcast_ref::<String>() {
            Some(v.to_owned())
        } else if let Some(v) = v.downcast_ref::<&str>() {
            Some(v.to_owned().to_owned())
        } else {
            None
        };
        let msg = msg.unwrap_or("unknown".to_owned());
        print!(
            "panic during prompt generation: {} at {}\n> ",
            msg,
            i.location().map(|l| l.to_string()).unwrap_or_default()
        )
    }));
    let _ = std::panic::catch_unwind(|| {
        let p = gen_prompt();
        print!("{}", p);
    });
}

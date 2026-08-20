use std::env;
use std::fs;
use std::process::ExitCode;

use cangjie_color::{Mode, load_graphics, load_recipes, teaching_aid_svg};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("cj-color: {e}");
            ExitCode::FAILURE
        }
    }
}

fn parse_mode(s: &str) -> Result<Mode, String> {
    if s.eq_ignore_ascii_case("full") {
        return Ok(Mode::Full);
    }
    if s.eq_ignore_ascii_case("grey") || s.eq_ignore_ascii_case("greyonly") {
        return Ok(Mode::GreyOnly);
    }
    if let Some(rest) = s.strip_prefix("letter:") {
        let ch = rest
            .chars()
            .next()
            .ok_or_else(|| "letter: needs a char".to_string())?;
        return Ok(Mode::Letter(ch.to_ascii_uppercase()));
    }
    if let Some(rest) = s.strip_prefix("unit:") {
        let i: usize = rest
            .parse()
            .map_err(|_| format!("bad unit index: {rest}"))?;
        return Ok(Mode::Unit(i));
    }
    Err(format!("unknown --mode {s} (full|letter:X|unit:N|grey)"))
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let recipes = flag_value(&args, "--recipes")?;
    let graphics = flag_value(&args, "--graphics")?;
    let ch_s = flag_value(&args, "--char")?;
    let out = flag_value(&args, "-o")?;

    let mode = match flag_value(&args, "--mode") {
        Ok(s) => parse_mode(s)?,
        Err(_) => Mode::Full,
    };

    let ch = ch_s
        .chars()
        .next()
        .ok_or_else(|| "empty --char".to_string())?;

    let recipes_map = load_recipes(recipes.as_ref())?;
    let graphics_map = load_graphics(graphics.as_ref())?;
    let recipe = recipes_map
        .get(&ch)
        .ok_or_else(|| format!("no recipe for {ch}"))?;
    let paths = graphics_map
        .get(&ch)
        .ok_or_else(|| format!("no graphics for {ch}"))?;

    let svg = teaching_aid_svg(recipe, paths, mode);
    fs::write(&out, svg).map_err(|e| format!("write {out}: {e}"))?;
    Ok(())
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Result<&'a String, String> {
    let i = args
        .iter()
        .position(|a| a == flag)
        .ok_or_else(|| format!("missing {flag}"))?;
    args.get(i + 1)
        .ok_or_else(|| format!("{flag} needs a value"))
}

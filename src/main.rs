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

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    let recipes = flag_value(&args, "--recipes")?;
    let graphics = flag_value(&args, "--graphics")?;
    let ch_s = flag_value(&args, "--char")?;
    let out = flag_value(&args, "-o")?;

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

    let svg = teaching_aid_svg(recipe, paths, Mode::Full);
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

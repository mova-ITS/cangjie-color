use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use cangjie_color::{Mode, Recipe, load_graphics, load_recipes, teaching_aid_svg};

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
    let recipes_path = flag_value(&args, "--recipes")?;
    let graphics_path = flag_value(&args, "--graphics")?;
    let mode = match flag_optional(&args, "--mode") {
        Some(s) => parse_mode(s)?,
        None => Mode::Full,
    };

    let out_dir = flag_optional(&args, "--out-dir");
    let out_file = flag_optional(&args, "-o");
    let one_char = flag_optional(&args, "--char");

    let recipes = load_recipes(Path::new(recipes_path))?;
    let graphics = load_graphics(Path::new(graphics_path))?;

    match (out_dir, out_file, one_char) {
        (Some(dir), None, None) => {
            let chars = resolve_batch_chars(&args, &recipes, &graphics)?;
            batch_write(dir, &chars, &recipes, &graphics, mode)
        }
        (None, Some(out), Some(ch_s)) => {
            let ch = ch_s
                .chars()
                .next()
                .ok_or_else(|| "empty --char".to_string())?;
            write_one(out, ch, &recipes, &graphics, mode)
        }
        _ => Err(
            "use either single (--char CHAR -o FILE) or batch (--out-dir DIR [--chars …|--chars-file …])"
                .into(),
        ),
    }
}

fn resolve_batch_chars(
    args: &[String],
    recipes: &HashMap<char, Recipe>,
    graphics: &HashMap<char, Vec<String>>,
) -> Result<Vec<char>, String> {
    if let Some(list) = flag_optional(args, "--chars") {
        return parse_chars_csv(list);
    }
    if let Some(path) = flag_optional(args, "--chars-file") {
        return load_chars_file(Path::new(path));
    }
    let mut chars: Vec<char> = recipes
        .keys()
        .copied()
        .filter(|c| graphics.contains_key(c))
        .collect();
    chars.sort_unstable();
    if chars.is_empty() {
        return Err("no characters in both recipes and graphics".into());
    }
    Ok(chars)
}

fn parse_chars_csv(s: &str) -> Result<Vec<char>, String> {
    let mut out = Vec::new();
    for part in s.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let mut chars = part.chars();
        let ch = chars
            .next()
            .ok_or_else(|| format!("empty entry in --chars: {s:?}"))?;
        if chars.next().is_some() {
            return Err(format!("--chars entry must be one character: {part:?}"));
        }
        out.push(ch);
    }
    if out.is_empty() {
        return Err("empty --chars".into());
    }
    Ok(out)
}

fn load_chars_file(path: &Path) -> Result<Vec<char>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut chars = line.chars();
        let ch = chars
            .next()
            .ok_or_else(|| format!("{}:{}: empty line", path.display(), i + 1))?;
        if chars.next().is_some() {
            return Err(format!(
                "{}:{}: expected one character per line, got {line:?}",
                path.display(),
                i + 1
            ));
        }
        out.push(ch);
    }
    if out.is_empty() {
        return Err(format!("no characters in {}", path.display()));
    }
    Ok(out)
}

fn batch_write(
    out_dir: &str,
    chars: &[char],
    recipes: &HashMap<char, Recipe>,
    graphics: &HashMap<char, Vec<String>>,
    mode: Mode,
) -> Result<(), String> {
    fs::create_dir_all(out_dir).map_err(|e| format!("mkdir {out_dir}: {e}"))?;
    for &ch in chars {
        let path = PathBuf::from(out_dir).join(format!("{ch}.svg"));
        let path_s = path.to_string_lossy();
        write_one(&path_s, ch, recipes, graphics, mode)?;
    }
    Ok(())
}

fn write_one(
    out: &str,
    ch: char,
    recipes: &HashMap<char, Recipe>,
    graphics: &HashMap<char, Vec<String>>,
    mode: Mode,
) -> Result<(), String> {
    let recipe = recipes
        .get(&ch)
        .ok_or_else(|| format!("no recipe for {ch}"))?;
    let paths = graphics
        .get(&ch)
        .ok_or_else(|| format!("no graphics for {ch}"))?;
    let svg = teaching_aid_svg(recipe, paths, mode);
    fs::write(out, svg).map_err(|e| format!("write {out}: {e}"))?;
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

fn flag_optional<'a>(args: &'a [String], flag: &str) -> Option<&'a String> {
    flag_value(args, flag).ok()
}

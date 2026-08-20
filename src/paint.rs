// paint.rs

use crate::recipe::{Mode, Recipe};

pub fn teaching_aid_svg(recipe: &Recipe, paths: &[String], mode: Mode) -> String {
    match mode {
        Mode::Full => paint_full(recipe, paths),
        Mode::Letter(letter) => paint_letter(recipe, paths, letter),
        Mode::Unit(i) => paint_unit(recipe, paths, i),
        Mode::GreyOnly => paint_grey_only(paths),
    }
}

fn paint_grey_only(paths: &[String]) -> String {
    let fills: Vec<String> = vec!["#222222".to_string(); paths.len()];
    svg_from_fills(paths, &fills)
}

fn paint_unit(recipe: &Recipe, paths: &[String], i: usize) -> String {
    let mut fills: Vec<String> = vec!["#222222".to_string(); paths.len()];
    if let Some(sign) = recipe.signs.get(i) {
        for &idx in &sign.strokes {
            if let Some(slot) = fills.get_mut(idx) {
                *slot = sign.color.clone();
            }
        }
    }
    svg_from_fills(paths, &fills)
}

fn paint_letter(recipe: &Recipe, paths: &[String], letter: char) -> String {
    let mut fills: Vec<String> = vec!["#222222".to_string(); paths.len()];
    if let Some(sign) = recipe.signs.iter().find(|s| s.letter == letter) {
        for &idx in &sign.strokes {
            if let Some(slot) = fills.get_mut(idx) {
                *slot = sign.color.clone();
            }
        }
    }
    svg_from_fills(paths, &fills)
}

fn paint_full(recipe: &Recipe, paths: &[String]) -> String {
    let mut fills: Vec<String> = vec!["#222222".to_string(); paths.len()];
    for sign in &recipe.signs {
        for &idx in &sign.strokes {
            if let Some(slot) = fills.get_mut(idx) {
                *slot = sign.color.clone();
            }
        }
    }
    svg_from_fills(paths, &fills)
}

/// Paths are expected already in SVG Y-down space (flipped at load).
fn svg_from_fills(paths: &[String], fills: &[String]) -> String {
    let mut out = String::from(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">"#,
    );
    for (d, fill) in paths.iter().zip(fills.iter()) {
        out.push_str("<path d='");
        out.push_str(d);
        out.push_str("' fill='");
        out.push_str(fill);
        out.push_str("'/>");
    }
    out.push_str("</svg>");
    out
}

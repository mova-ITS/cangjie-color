//! cangjie-color — teaching-aid SVGs from Cangjie stroke colour recipes.
//!
//! # Product concept (one sentence)
//! Teaching aids (as SVG) that show curated Cangjie letter→stroke colouring
//! on a character — not inventing the code.
//!
//! # Data flow vs TDD
//! `mock → teaching_aid_svg → SVG` is **data**, not TDD.
//!
//! # TDD flow (how we build)
//! 1. Write ONE failing test     ← red  
//! 2. Write minimal code         ← green  
//! 3. Refactor if needed  
//! 4. Next test  
//!
//! # MVP slice (now)
//! In-memory `mock_zi()` → `teaching_aid_svg` → SVG string. No files yet.
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sign {
    pub letter: char,
    pub color: String,
    pub strokes: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Recipe {
    pub code: String,
    pub signs: Vec<Sign>,
}

pub fn load_graphics(path: &Path) -> Result<HashMap<char, Vec<String>>, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;

    let mut map = HashMap::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let v: serde_json::Value =
            serde_json::from_str(line).map_err(|e| format!("json line {}: {e}", i + 1))?;
        let ch = v["character"]
            .as_str()
            .and_then(|s| s.chars().next())
            .ok_or_else(|| format!("line {}: missing character", i + 1))?;
        let strokes = v["strokes"]
            .as_array()
            .ok_or_else(|| format!("line {}: missing strokes", i + 1))?
            .iter()
            .map(|s| {
                s.as_str()
                    .map(str::to_string)
                    .ok_or_else(|| format!("line {}: stroke not a string", i + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;
        map.insert(ch, strokes);
    }
    Ok(map)
}

pub fn load_recipes(path: &Path) -> Result<HashMap<char, Recipe>, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;

    let mut map = HashMap::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // JSON has 'char' and 'code'. We only need those for this test
        let v: serde_json::Value =
            serde_json::from_str(line).map_err(|e| format!("json line {}: {e}", i + 1))?;

        let ch = v["char"]
            .as_str()
            .and_then(|s| s.chars().next())
            .ok_or_else(|| format!("line {}: missing char", i + 1))?;

        let code = v["code"]
            .as_str()
            .ok_or_else(|| format!("line {}: missing code", i + 1))?
            .to_string();

        let signs = v["signs"]
            .as_array()
            .ok_or_else(|| format!("line {}: missing signs", i + 1))?
            .iter()
            .enumerate()
            .map(|(j, s)| {
                let letter = s["letter"]
                    .as_str()
                    .and_then(|t| t.chars().next())
                    .ok_or_else(|| format!("line {} sign {}: bad letter", i + 1, j))?;
                let color = s["color"]
                    .as_str()
                    .ok_or_else(|| format!("line {} sign {}: bad color", i + 1, j))?
                    .to_string();
                let strokes = s["strokes"]
                    .as_array()
                    .ok_or_else(|| format!("line {} sign {}: bad strokes", i + 1, j))?
                    .iter()
                    .map(|n| {
                        n.as_u64()
                            .map(|u| u as usize)
                            .ok_or_else(|| format!("line {} sign {}: bad stroke idx", i + 1, j))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Sign {
                    letter,
                    color,
                    strokes,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        map.insert(ch, Recipe { code, signs });
    }
    Ok(map)
}

pub fn teaching_aid_svg(recipe: &Recipe, paths: &[String]) -> String {
    // Default ink, then overwrite from signs (same idea as the Python painter).
    let mut fills: Vec<String> = vec!["#222222".to_string(); paths.len()];
    for sign in &recipe.signs {
        for &idx in &sign.strokes {
            if let Some(slot) = fills.get_mut(idx) {
                *slot = sign.color.clone();
            }
        }
    }

    let mut out = String::from("<svg>");
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

#[cfg(test)]
mod tests {

    use super::{load_graphics, load_recipes, teaching_aid_svg, Recipe};

    use std::path::PathBuf;

    fn graphics_sample() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/graphics/sample.jsonl")
    }

    fn recipes_sample() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/recipes/sample.jsonl")
    }

    fn loaded_zi() -> (Recipe, Vec<String>) {
        let recipes = load_recipes(&recipes_sample()).expect("recipes");
        let graphics = load_graphics(&graphics_sample()).expect("graphics");
        let zi = recipes.get(&'子').expect("子 recipe").clone();
        let paths = graphics.get(&'子').expect("子 paths").clone();
        (zi, paths)
    }

    #[test]
    fn teaching_aid_contains_svg_root() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths);
        assert!(svg.contains("<svg"), "got {svg:?}");
    }
    #[test]
    fn teaching_aid_emits_one_path_per_stroke() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths);
        assert_eq!(svg.matches("<path").count(), paths.len());
    }
    #[test]
    fn teaching_aid_uses_letter_colours() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths);
        assert!(svg.contains("#1a9e3b"), "N green missing: {svg}");
        assert!(svg.contains("#e6194b"), "D red missing: {svg}");
    }

    #[test]
    fn load_graphics_sample_zi_has_three_strokes() {
        let graphics = load_graphics(&graphics_sample()).expect("graphics sample must laod");
        let paths = graphics.get(&'子').expect("it must be present");

        assert!(
            paths.len() >= 3,
            "子 needs >= stroke paths, got {}",
            paths.len()
        );
    }

    #[test]
    fn load_recipes_sample_has_zi_as_nd() {
        let recipes = load_recipes(&recipes_sample()).expect("recipes sample must load");
        let zi = recipes.get(&'子').expect("子 must be present");
        assert_eq!(zi.code, "ND");
    }

    #[test]
    fn load_recipes_zi_has_nd_signs() {
        let recipes = load_recipes(&recipes_sample()).expect("load recipes");
        let zi = recipes.get(&'子').expect("子");
        assert_eq!(zi.signs.len(), 2);
        assert_eq!(zi.signs[0].letter, 'N');
        assert_eq!(zi.signs[0].color, "#1a9e3b");
        assert_eq!(zi.signs[0].strokes, vec![0]);
        assert_eq!(zi.signs[1].letter, 'D');
        assert_eq!(zi.signs[1].color, "#e6194b");
        assert_eq!(zi.signs[1].strokes, vec![1, 2]);
    }

    #[test]
    fn teaching_aid_from_loaded_zi_shows_letter_colours() {
        let recipes = load_recipes(&recipes_sample()).expect("recipes");
        let graphics = load_graphics(&graphics_sample()).expect("graphics");
        let zi = recipes.get(&'子').expect("子 recipe");
        let paths = graphics.get(&'子').expect("子 paths");

        let svg = teaching_aid_svg(zi, paths);

        assert!(svg.contains("<svg"), "got {svg:?}");
        assert_eq!(svg.matches("<path").count(), paths.len());
        assert!(svg.contains("#1a9e3b"), "N green missing: {svg}");
        assert!(svg.contains("#e6194b"), "D red missing: {svg}");
    }
}

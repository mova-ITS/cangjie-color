// load.rs
use std::collections::HashMap;
use std::path::Path;

use crate::path_flip::flip_mmah_path;
use crate::recipe::{Recipe, Sign};

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
            .enumerate()
            .map(|(si, s)| {
                let raw = s
                    .as_str()
                    .ok_or_else(|| format!("line {}: stroke {si} not a string", i + 1))?;
                flip_mmah_path(raw).map_err(|e| format!("line {} stroke {si}: {e}", i + 1))
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

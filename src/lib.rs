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
mod load;
mod paint;
mod path_flip;
mod recipe;

pub use load::{load_graphics, load_recipes};
pub use paint::teaching_aid_svg;
pub use recipe::{Mode, Recipe, Sign};

#[cfg(test)]
mod tests {

    use super::{Mode, Recipe, load_graphics, load_recipes, teaching_aid_svg};

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
    fn teaching_aid_svg_is_browser_ready() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths, Mode::Full);
        assert!(
            svg.contains(r#"xmlns="http://www.w3.org/2000/svg""#),
            "missing xmlns: {svg}"
        );
        assert!(
            svg.contains(r#"viewBox="0 0 1024 1024""#),
            "missing viewBox: {svg}"
        );
        assert!(!svg.contains("transform="), "no transform wanted: {svg}");
        assert!(!svg.contains("1024 -1024"), "no negative viewBox: {svg}");
    }

    #[test]
    fn teaching_aid_grey_only_has_no_letter_colours() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths, Mode::GreyOnly);
        assert!(!svg.contains("#1a9e3b"), "N green must be gone: {svg}");
        assert!(!svg.contains("#e6194b"), "D red must be gone: {svg}");
        assert!(svg.contains("#222222"), "dim fill missing: {svg}");
    }

    #[test]
    fn teaching_aid_unit_0_keeps_green_not_red() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths, Mode::Unit(0));
        assert!(svg.contains("#1a9e3b"), "unit0 N green missing: {svg}");
        assert!(
            !svg.contains("#e6194b"),
            "unit1 D must not stay red in Unit(0): {svg}"
        );
    }

    #[test]
    fn teaching_aid_letter_n_keeps_green_not_red() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths, Mode::Letter('N'));
        assert!(svg.contains("#1a9e3b"), "N green missing: {svg}");
        assert!(
            !svg.contains("#e6194b"),
            "D must not stay red in Letter(N): {svg}"
        );
    }

    #[test]
    fn teaching_aid_contains_svg_root() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths, Mode::Full);
        assert!(svg.contains("<svg"), "got {svg:?}");
    }
    #[test]
    fn teaching_aid_emits_one_path_per_stroke() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths, Mode::Full);
        assert_eq!(svg.matches("<path").count(), paths.len());
    }
    #[test]
    fn teaching_aid_uses_letter_colours() {
        let (zi, paths) = loaded_zi();
        let svg = teaching_aid_svg(&zi, &paths, Mode::Full);
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

        let svg = teaching_aid_svg(zi, paths, Mode::Full);

        assert!(svg.contains("<svg"), "got {svg:?}");
        assert_eq!(svg.matches("<path").count(), paths.len());
        assert!(svg.contains("#1a9e3b"), "N green missing: {svg}");
        assert!(svg.contains("#e6194b"), "D red missing: {svg}");
    }

    #[test]
    fn load_recipes_missing_file_is_err() {
        let path =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("testdata/recipes/does-not-exist.jsonl");
        let err = load_recipes(&path).expect_err("missing file must Err");
        assert!(
            err.contains("read")
                || err.contains("No such")
                || err.contains("not found")
                || path.to_string_lossy().contains("does-not-exist")
                || err.contains("does-not-exist"),
            "error should mention read/path failure, got: {err}"
        );
    }

    #[test]
    fn load_recipes_bad_json_line_is_err() {
        let dir = std::env::temp_dir();
        let path = dir.join("cangjie-color-bad-recipes.jsonl");
        std::fs::write(&path, "{not valid json\n").expect("write temp");

        let err = load_recipes(&path).expect_err("bad JSON must Err");
        assert!(
            err.contains("json") || err.contains("line"),
            "error should mention json/line, got: {err}"
        );
    }

    #[test]
    fn load_graphics_missing_file_is_err() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("testdata/graphics/does-not-exist.jsonl");
        let err = load_graphics(&path).expect_err("missing file must Err");
        assert!(
            err.contains("read")
                || err.contains("No such")
                || err.contains("not found")
                || err.contains("does-not-exist"),
            "got: {err}"
        );
    }

    #[test]
    fn load_graphics_bad_json_line_is_err() {
        let path = std::env::temp_dir().join("cangjie-color-bad-graphics.jsonl");
        std::fs::write(&path, "{not valid json\n").expect("write temp");
        let err = load_graphics(&path).expect_err("bad JSON must Err");
        assert!(err.contains("json") || err.contains("line"), "got: {err}");
    }
}

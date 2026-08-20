//! Flip MMAH path `d` strings into SVG Y-down space (no SVG transform).
//!
//! Make Me a Hanzi uses a 1024×1024 system where Y *decreases* downward
//! (upper-left ≈ (0, 900), lower-right ≈ (1024, −124)). Standard SVG has
//! Y increasing downward. Official MMAH display uses
//! `scale(1,-1) translate(0,-900)`; the equivalent rewrite is:
//! `y_svg = 900 - y_mmah`.
//!
//! Scope v1: absolute `M` / `L` / `Q` / `C` / `Z` only.

/// MMAH reference top (see makemeahanzi README). Not the em box size.
pub const MMAH_Y_TOP: f64 = 900.0;

pub fn flip_mmah_path(d: &str) -> Result<String, String> {
    flip_mmah_path_box(d, MMAH_Y_TOP)
}

pub fn flip_mmah_path_box(d: &str, y_top: f64) -> Result<String, String> {
    let tokens = tokenize(d)?;
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Tok::Cmd(c) => {
                let cmd = *c;
                if !matches!(cmd, 'M' | 'L' | 'Q' | 'C' | 'Z') {
                    return Err(format!(
                        "unsupported path command '{cmd}' (MMAH absolute M/L/Q/C/Z only)"
                    ));
                }
                out.push(cmd.to_string());
                i += 1;
                if cmd == 'Z' {
                    continue;
                }
                while i + 1 < tokens.len() {
                    let (Tok::Num(_), Tok::Num(_)) = (&tokens[i], &tokens[i + 1]) else {
                        break;
                    };
                    let Tok::Num(x) = tokens[i] else { unreachable!() };
                    let Tok::Num(y) = tokens[i + 1] else { unreachable!() };
                    out.push(fmt_num(x));
                    out.push(fmt_num(y_top - y));
                    i += 2;
                }
            }
            Tok::Num(n) => {
                return Err(format!("orphan number {n} before a command in path"));
            }
        }
    }
    Ok(out.join(" "))
}

#[derive(Debug)]
enum Tok {
    Cmd(char),
    Num(f64),
}

fn tokenize(d: &str) -> Result<Vec<Tok>, String> {
    let mut out = Vec::new();
    let bytes = d.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b.is_ascii_whitespace() || b == b',' {
            i += 1;
            continue;
        }
        if b.is_ascii_alphabetic() {
            out.push(Tok::Cmd(b as char));
            i += 1;
            continue;
        }
        // number: optional sign, digits, optional fraction
        let start = i;
        if bytes[i] == b'+' || bytes[i] == b'-' {
            i += 1;
        }
        let mut saw_digit = false;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            saw_digit = true;
            i += 1;
        }
        if i < bytes.len() && bytes[i] == b'.' {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                saw_digit = true;
                i += 1;
            }
        }
        if !saw_digit {
            return Err(format!("bad number at byte {start} in path"));
        }
        let s = std::str::from_utf8(&bytes[start..i]).map_err(|e| e.to_string())?;
        let n: f64 = s
            .parse()
            .map_err(|e| format!("bad number '{s}': {e}"))?;
        out.push(Tok::Num(n));
    }
    Ok(out)
}

fn fmt_num(n: f64) -> String {
    if (n - n.round()).abs() < 1e-9 {
        format!("{}", n.round() as i64)
    } else {
        let s = format!("{n:.4}");
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flips_moveto_y() {
        // MMAH: y_svg = 900 - y
        let out = flip_mmah_path("M 0 100").expect("flip");
        assert_eq!(out, "M 0 800");
    }

    #[test]
    fn flips_quad_control_and_end() {
        let out = flip_mmah_path("M 10 20 Q 30 40 50 60").expect("flip");
        assert_eq!(out, "M 10 880 Q 30 860 50 840");
    }

    #[test]
    fn rejects_relative_commands() {
        let err = flip_mmah_path("m 0 100").expect_err("relative");
        assert!(err.contains("unsupported"), "{err}");
    }
}

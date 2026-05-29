use regex::Regex;

/// A segment of text with its associated Minecraft format codes
#[derive(Debug, Clone)]
pub struct TextSegment {
    pub format_codes: Vec<char>,
    pub text: String,
}

/// Parse lore lines into segments with format codes
pub fn parse_lore_lines(lines: &[&str], pattern: &Regex) -> Vec<Vec<TextSegment>> {
    lines
        .iter()
        .map(|line| {
            pattern
                .captures_iter(line)
                .map(|caps| TextSegment {
                    format_codes: split_format_codes(&caps[1]),
                    text: caps[2].to_string(),
                })
                .collect()
        })
        .collect()
}

/// Split a string of format codes like "§a§l" into vec!["§a", "§l"]
pub fn split_format_codes(input: &str) -> Vec<char> {
    input.chars().skip(1).step_by(2).collect()
}

/// Parse a Roman numeral (I, V, X only) into an integer
/// Valid range: 1-10 for enchantment tiers
pub fn parse_roman_numeral(s: &str) -> Option<i32> {
    let mut total = 0;
    let mut prev = 0;

    for c in s.chars() {
        let val = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            _ => return None,
        };

        if val > prev {
            // Subtractive notation: subtract twice the previous (since we already added it once)
            total += val - 2 * prev;
        } else {
            total += val;
        }
        prev = val;
    }

    match total {
        1..=10 => Some(total),
        _ => None,
    }
}
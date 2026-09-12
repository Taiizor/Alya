use std::collections::BTreeMap;

pub fn strip_toml_comment(line: &str) -> &str {
    let mut in_quote = false;
    let mut quote_char = ' ';
    for (i, c) in line.char_indices() {
        if (c == '"' || c == '\'') && (i == 0 || line.as_bytes()[i - 1] != b'\\') {
            if in_quote && c == quote_char {
                in_quote = false;
            } else if !in_quote {
                in_quote = true;
                quote_char = c;
            }
        } else if c == '#' && !in_quote {
            return line[..i].trim();
        }
    }
    line.trim()
}

pub fn unquote(s: &str) -> String {
    let trimmed = s.trim();
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
    {
        if trimmed.len() >= 2 {
            trimmed[1..trimmed.len() - 1].to_string()
        } else {
            String::new()
        }
    } else {
        trimmed.to_string()
    }
}

pub fn parse_string_array(s: &str) -> Vec<String> {
    let trimmed = s.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Vec::new();
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    inner
        .split(',')
        .map(|item| unquote(item.trim()))
        .filter(|item| !item.is_empty())
        .collect()
}

pub fn parse_inline_table(s: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    let trimmed = s.trim();
    if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
        return map;
    }
    let inner = &trimmed[1..trimmed.len() - 1];
    for part in inner.split(',') {
        if let Some((k, v)) = part.split_once('=') {
            map.insert(k.trim().to_string(), unquote(v.trim()));
        }
    }
    map
}

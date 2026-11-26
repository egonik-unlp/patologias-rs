//! Autocomplete suggestions: the `suggestions!` query macro and the ranking
//! of its raw `(value, count)` rows.
use std::collections::HashMap;

/// Values that differ only in case, a leading dash, a trailing period or spaces
/// get the same key.
fn normalize_key(s: &str) -> String {
    s.trim_start_matches(|c: char| c == '-' || c.is_whitespace())
        .trim_end_matches(|c: char| c == '.' || c.is_whitespace())
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// `rows` = (exact value, times used). Returns the most common spelling
/// of each group, most-used groups first.
pub fn rank_suggestions<I>(rows: I, limit: usize) -> Vec<String>
where
    I: IntoIterator<Item = (String, i64)>,
{
    // key -> (spelling -> count)
    let mut groups: HashMap<String, HashMap<String, i64>> = HashMap::new();
    for (raw, n) in rows {
        let value = raw.trim();
        let key = normalize_key(value);
        if key.is_empty() {
            continue; // blank, "-", "." …
        }
        *groups
            .entry(key)
            .or_default()
            .entry(value.to_owned())
            .or_default() += n;
    }

    let mut ranked: Vec<(i64, String)> = groups
        .into_values()
        .map(|spellings| {
            let total = spellings.values().sum();
            let best = spellings
                .into_iter()
                .max_by(|(a, na), (b, nb)| na.cmp(nb).then_with(|| b.cmp(a)))
                .map(|(v, _)| v)
                .expect("a group always has at least one spelling");
            (total, best)
        })
        .collect();

    ranked.sort_by(|(ta, a), (tb, b)| tb.cmp(ta).then_with(|| a.cmp(b)));
    ranked.into_iter().take(limit).map(|(_, v)| v).collect()
}
#[macro_export]
macro_rules! suggestions {
    (unique, $conn:ident, $table_name:ident, $col:expr) => {{
        use ::diesel::{QueryDsl as _, RunQueryDsl as _};
        $table_name::table
            .group_by($col)
            .select(($col, ::diesel::dsl::count_star()))
            .load::<(String, i64)>($conn)
    }};
    ($conn:ident, $table_name:ident, $col:expr) => {{
        use ::diesel::{ExpressionMethods as _, QueryDsl as _, RunQueryDsl as _};
        $table_name::table
            .filter($col.is_not_null())
            .group_by($col)
            .select(($col, ::diesel::dsl::count_star()))
            .load::<(Option<String>, i64)>($conn)
    }};
}

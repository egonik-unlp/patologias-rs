use crate::core::database::connection::{get_connection_pool, interact_with_connection};
use crate::core::database::repository;
use crate::core::database::schema::{citologias, histopatologias, studies};
use crate::impl_repository;
use crate::reports::dto::{CitoFields, FieldsDto, HistoFields};
use crate::reports::model::{
    Citologia, Histopatologia, NewCitologia, NewHistopatologia, NewStudy, Study, StudyType,
};
use std::collections::HashMap;

impl_repository!(
    async, Study, studies, i32, NewStudy,
    children(study_id) {
        Citologia: citologias, NewCitologia;
        Histopatologia: histopatologias, NewHistopatologia;
    },
    kind = tipo: StudyType,
);

use diesel::prelude::*;
macro_rules! suggestions {
    ($conn:ident, $table_name: ident, $col: expr) => {
        $table_name::table
            .filter($col.is_not_null())
            .group_by($col)
            .select(($col, diesel::dsl::count_star()))
            .load::<(Option<String>, i64)>($conn)
    };
}

impl StudyRepository {
    pub async fn get_suggestions(
        &self,
        field: FieldsDto,
        limit: usize,
    ) -> anyhow::Result<Vec<String>> {
        let raw_suggestions = interact_with_connection(&self.pool, |conn| match field {
            FieldsDto::Citologia(cito_fields) => match cito_fields {
                CitoFields::Microflora => suggestions!(conn, citologias, citologias::microflora),
                CitoFields::CelulasPavimentosas => {
                    suggestions!(conn, citologias, citologias::celulas_pavimentosas)
                }
                CitoFields::CelulasMetaplasicas => {
                    suggestions!(conn, citologias, citologias::celulas_pavimentosas)
                }
                CitoFields::Leucocitos => {
                    suggestions!(conn, citologias, citologias::celulas_pavimentosas)
                }
                CitoFields::Hematies => {
                    suggestions!(conn, citologias, citologias::celulas_pavimentosas)
                }
                CitoFields::CelulasEndocervicales => {
                    suggestions!(conn, citologias, citologias::celulas_endocervicales)
                }
                CitoFields::CompatibleCon => {
                    suggestions!(conn, citologias, citologias::compatible_con)
                }
                CitoFields::Sugerencia => suggestions!(conn, citologias, citologias::sugerencia),
                CitoFields::DiagCitologias => {
                    suggestions!(conn, citologias, citologias::sugerencia)
                }
            },
            FieldsDto::Histopatologia(histo_fields) => match histo_fields {
                HistoFields::DescripcionMacroscopica => suggestions!(
                    conn,
                    histopatologias,
                    histopatologias::descripcion_macroscopica
                ),
                HistoFields::DiagnosticoHistopatologico => suggestions!(
                    conn,
                    histopatologias,
                    histopatologias::descripcion_macroscopica
                ),
            },
        })
        .await?;
        Ok(rank_suggestions(
            raw_suggestions
                .into_iter()
                .filter_map(|(v, n)| Some((v?, n))),
            limit,
        ))
    }
}

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

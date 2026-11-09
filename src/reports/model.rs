use chrono::NaiveDateTime;
use diesel::{
    associations::Identifiable, deserialize::Queryable, prelude::Insertable,
    query_builder::AsChangeset, Selectable,
};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

pub enum StudyDetail {
    Histopatologia(Histopatologia),
    Citologia(Citologia),
}

#[derive(Debug, Deserialize, Serialize, DbEnum, Default)]
#[serde(rename_all = "lowercase")]
#[ExistingTypePath = "crate::core::database::schema::sql_types::StudyType"]
pub enum StudyType {
    Histopatologia,
    #[default]
    Citologia,
}
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::citologias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Citologia {
    id: i32,
    study_id: i32,
    microflora: Option<String>,
    celulas_pavimentosas: Option<String>,
    celulas_metaplasicas: Option<String>,
    leucocitos: Option<String>,
    hematies: Option<String>,
    celulas_endocervicales: Option<String>,
    compatible_con: Option<String>,
    sugerencia: Option<String>,
    diag_citologias: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset, Default)]
#[diesel(table_name= crate::core::database::schema::citologias)]
pub struct NewCitologia {
    pub study_id: Option<i32>,
    pub microflora: Option<String>,
    pub celulas_pavimentosas: Option<String>,
    pub celulas_metaplasicas: Option<String>,
    pub leucocitos: Option<String>,
    pub hematies: Option<String>,
    pub celulas_endocervicales: Option<String>,
    pub compatible_con: Option<String>,
    pub sugerencia: Option<String>,
    pub diag_citologias: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::histopatologias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Histopatologia {
    id: i32,
    study_id: i32,
    descripcion_macroscopica: Option<String>,
    diagnostico_histopatologico: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable, Default)]
#[diesel(table_name= crate::core::database::schema::histopatologias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewHistopatologia {
    pub study_id: Option<i32>,
    pub descripcion_macroscopica: Option<String>,
    #[serde(alias = "diagnostico_histopat")]
    pub diagnostico_histopatologico: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::studies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Study {
    id: i32,
    protocolo: i32,
    fecha: NaiveDateTime,
    tipo: StudyType,
    paciente: String,
    doctor_id: i32,
    material_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset, Default)]
#[diesel(table_name= crate::core::database::schema::studies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStudy {
    pub protocolo: i32,
    #[serde(deserialize_with = "deserialize_fecha")]
    pub fecha: NaiveDateTime,
    pub tipo: StudyType,
    pub paciente: String,
    // The CSV has doctor/material *names* (`medico`, `material`), not ids, so
    // these default to 0 when deserializing; the loader must resolve the names
    // to real ids before inserting.
    #[serde(default)]
    pub doctor_id: i32,
    #[serde(default)]
    pub material_id: i32,
}

/// Accepts both ISO-8601 (`2008-01-02T21:21:46`) and the legacy
/// space-separated format (`2008-01-02 21:21:46`).
fn deserialize_fecha<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<NaiveDateTime>()
        .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f"))
        .map_err(serde::de::Error::custom)
}

use chrono::NaiveDateTime;
use diesel::{
    associations::Identifiable, deserialize::Queryable, prelude::Insertable,
    query_builder::AsChangeset, Selectable,
};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

use crate::reports::{
    dto::{CitologiaDto, FullStudyDto, HistopatologiaDto, StudyDetailDto, StudyDto, StudyTypeDto},
    repository::{NewStudyDetail, StudyDetail},
};

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
#[diesel(table_name = crate::core::database::schema::citologias)]
pub struct NewCitologia {
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
#[diesel(table_name = crate::core::database::schema::histopatologias)]
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
    pub descripcion_macroscopica: Option<String>,
    #[serde(alias = "diagnostico_histopat")]
    pub diagnostico_histopatologico: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::studies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Study {
    id: i32,
    protocolo: String,
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
    pub protocolo: String,
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

impl From<StudyTypeDto> for StudyType {
    fn from(value: StudyTypeDto) -> Self {
        match value {
            StudyTypeDto::Histopatologia => StudyType::Histopatologia,
            StudyTypeDto::Citologia => StudyType::Citologia,
        }
    }
}
impl From<StudyDto> for NewStudy {
    fn from(value: StudyDto) -> Self {
        let StudyDto {
            protocolo,
            fecha,
            paciente,
            doctor_id,
            tipo,
            material_id,
        } = value;
        let tipo = tipo.into();
        Self {
            protocolo,
            fecha,
            tipo,
            paciente,
            doctor_id: doctor_id,
            material_id: material_id,
        }
    }
}
pub type Parts = (Study, StudyDetail);
impl From<Parts> for FullStudyDto {
    fn from(value: Parts) -> Self {
        let (new_study, new_study_detail) = value;
        let tipi = match new_study_detail {
            StudyDetail::Citologia(_) => StudyTypeDto::Citologia,
            StudyDetail::Histopatologia(_) => StudyTypeDto::Histopatologia,
        };
        let Study {
            protocolo,
            fecha,
            tipo,
            paciente,
            doctor_id,
            material_id,
            ..
        } = new_study;
        let parent = StudyDto {
            doctor_id,
            protocolo,
            fecha,
            tipo: tipi,
            paciente,
            material_id,
        };
        let child = match new_study_detail {
            StudyDetail::Citologia(cito) => {
                let Citologia {
                    microflora,
                    celulas_pavimentosas,
                    celulas_metaplasicas,
                    leucocitos,
                    hematies,
                    celulas_endocervicales,
                    compatible_con,
                    sugerencia,
                    diag_citologias,
                    ..
                } = cito;
                StudyDetailDto::Citologia(CitologiaDto {
                    microflora,
                    celulas_pavimentosas,
                    celulas_metaplasicas,
                    leucocitos,
                    hematies,
                    celulas_endocervicales,
                    compatible_con,
                    sugerencia,
                    diag_citologias,
                })
            }
            StudyDetail::Histopatologia(histo) => {
                let Histopatologia {
                    descripcion_macroscopica,
                    diagnostico_histopatologico,
                    ..
                } = histo;
                StudyDetailDto::Histopatologia(HistopatologiaDto {
                    descripcion_macroscopica,
                    diagnostico_histopatologico,
                })
            }
        };
        Self { parent, child }
    }
}

pub type NewParts = (NewStudy, NewStudyDetail);
impl FullStudyDto {
    pub fn into_parts(self) -> NewParts {
        (self.parent.into(), self.child.into_inner())
    }
}

impl StudyDetailDto {
    fn into_inner(self) -> NewStudyDetail {
        match self {
            Self::Citologia(dto) => NewStudyDetail::Citologia(dto.into()),
            Self::Histopatologia(dto) => NewStudyDetail::Histopatologia(dto.into()),
        }
    }
}
impl From<HistopatologiaDto> for NewHistopatologia {
    fn from(value: HistopatologiaDto) -> Self {
        let HistopatologiaDto {
            descripcion_macroscopica,
            diagnostico_histopatologico,
        } = value;
        Self {
            descripcion_macroscopica,
            diagnostico_histopatologico,
        }
    }
}

impl From<CitologiaDto> for NewCitologia {
    fn from(value: CitologiaDto) -> Self {
        let CitologiaDto {
            microflora,
            celulas_pavimentosas,
            celulas_metaplasicas,
            leucocitos,
            hematies,
            celulas_endocervicales,
            compatible_con,
            sugerencia,
            diag_citologias,
        } = value;
        Self {
            microflora,
            celulas_pavimentosas,
            celulas_metaplasicas,
            leucocitos,
            hematies,
            celulas_endocervicales,
            compatible_con,
            sugerencia,
            diag_citologias,
        }
    }
}

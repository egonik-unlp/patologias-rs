use std::{fmt::Display, str::FromStr};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FullStudyDto {
    pub parent: StudyDto,
    pub child: StudyDetailDto,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewFullStudyDto {
    pub parent: NewStudyDto,
    pub child: StudyDetailDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StudyDto {
    pub id: i32,
    pub protocolo: String,
    pub fecha: NaiveDateTime,
    pub paciente: String,
    pub doctor_id: i32,
    pub material_id: i32,
    pub tipo: StudyTypeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewStudyDto {
    pub protocolo: String,
    #[serde(deserialize_with = "de_fecha")]
    pub fecha: NaiveDateTime,
    pub paciente: String,
    pub doctor_id: i32,
    pub material_id: i32,
    pub tipo: StudyTypeDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudyDetailDto {
    Histopatologia(HistopatologiaDto),
    Citologia(CitologiaDto),
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub enum StudyTypeDto {
    Histopatologia,
    #[default]
    Citologia,
}

impl Display for StudyTypeDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            StudyTypeDto::Histopatologia => "Histopatologia",
            StudyTypeDto::Citologia => "Citologia",
        };
        write!(f, "{}", s)
    }
}
fn de_fecha<'de, D: serde::Deserializer<'de>>(d: D) -> Result<NaiveDateTime, D::Error> {
    let s = String::deserialize(d)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M") // what datetime-local sends
        .or_else(|_| s.parse()) // full format, so JSON etc. still work
        .inspect_err(|error| leptos::logging::log!("Parsing error: {}", error))
        .inspect_err(|error| println!("Parsing error: {}", error))
        .map_err(serde::de::Error::custom)
}
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CitologiaDto {
    pub microflora: String,
    pub celulas_pavimentosas: String,
    pub celulas_metaplasicas: String,
    pub leucocitos: String,
    pub hematies: String,
    pub celulas_endocervicales: String,
    pub compatible_con: String,
    pub sugerencia: String,
    pub diag_citologias: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewCitologiaDto {
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

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct HistopatologiaDto {
    pub descripcion_macroscopica: String,
    pub diagnostico_histopatologico: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewHistopatologiaDto {
    pub descripcion_macroscopica: Option<String>,
    pub diagnostico_histopatologico: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CitoFields {
    Microflora,
    CelulasPavimentosas,
    CelulasMetaplasicas,
    Leucocitos,
    Hematies,
    CelulasEndocervicales,
    CompatibleCon,
    Sugerencia,
    DiagCitologias,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HistoFields {
    DescripcionMacroscopica,
    DiagnosticoHistopatologico,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ReportFieldsDto {
    Citologia(CitoFields),
    Histopatologia(HistoFields),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseReportTypeError;

impl FromStr for StudyTypeDto {
    type Err = ParseReportTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s {
            "Histopatologia" | "histopatologia" => Ok(StudyTypeDto::Histopatologia),
            "Citologia" | "citologia" => Ok(StudyTypeDto::Citologia),
            _ => Err(ParseReportTypeError),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaginatedResponseDto {
    pub reports: Vec<StudyDto>,
    pub offset: i64,
    pub total: i64,
}

impl PaginatedResponseDto {
    pub fn new(
        reports: impl IntoIterator<Item = impl Into<StudyDto>>,
        offset: i64,
        total: i64,
    ) -> Self {
        let reports = reports.into_iter().map(Into::into).collect();
        PaginatedResponseDto {
            reports,
            offset,
            total,
        }
    }
}

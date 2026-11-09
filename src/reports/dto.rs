use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FullStudyDto {
    pub parent: StudyDto,
    pub child: StudyDetailDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StudyDetailDto {
    Histopatologia(HistopatologiaDto),
    Citologia(CitologiaDto),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum StudyTypeDto {
    Histopatologia,
    Citologia,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CitologiaDto {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistopatologiaDto {
    pub descripcion_macroscopica: Option<String>,
    pub diagnostico_histopatologico: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudyDto {
    pub protocolo: String,
    pub fecha: NaiveDateTime,
    pub paciente: String,
    pub doctor_id: i32,
    pub material_id: i32,
    pub tipo: StudyTypeDto,
}

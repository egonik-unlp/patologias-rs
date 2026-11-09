use chrono::NaiveDateTime;

use crate::{
    core::database::repository::{Child, Parent, ParentWithChildNew},
    reports::model::{NewCitologia, NewHistopatologia, NewStudy},
};

#[derive(Debug, Clone)]
pub struct FullStudyDto {
    parent: StudyDto,
    child: StudyDetail,
}

#[derive(Debug, Clone)]
pub enum StudyDetail {
    Histopatologia(HistopatologiaDto),
    Citologia(CitologiaDto),
}

#[derive(Debug, Clone)]
pub struct CitologiaDto {
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
            study_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistopatologiaDto {
    descripcion_macroscopica: Option<String>,
    diagnostico_histopatologico: Option<String>,
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
            study_id: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StudyDto {
    protocolo: i32,
    fecha: NaiveDateTime,
    paciente: String,
}

impl Child for StudyDetail {}
impl Parent for StudyDto {}

impl ParentWithChildNew for FullStudyDto {
    fn get_child(&self) -> impl crate::core::database::repository::Child {
        self.child.clone()
    }
    fn get_parent(&self) -> impl crate::core::database::repository::Parent {
        self.parent.clone()
    }
}

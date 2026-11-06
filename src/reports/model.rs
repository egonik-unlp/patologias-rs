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

#[derive(Debug, Deserialize, Serialize, DbEnum)]
#[ExistingTypePath = "crate::core::database::schema::sql_types::StudyType"]
pub enum StudyType {
    Histopatologia,
    Citologia,
}
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::citologias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Citologia {
    id: i32,
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

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name= crate::core::database::schema::citologias)]
pub struct NewCitologia {
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
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::histopatologias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Histopatologia {
    id: i32,
    study_id: i32,
    descripcion_macroscopica: Option<String>,
    diagnostio_histopatologico: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name= crate::core::database::schema::histopatologias)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewHistopatologia {
    study_id: i32,
    descripcion_macroscopica: Option<String>,
    diagnostio_histopatologico: Option<String>,
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
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name= crate::core::database::schema::studies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewStudy {
    protocolo: String,
    fecha: NaiveDateTime,
    tipo: StudyType,
    paciente: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

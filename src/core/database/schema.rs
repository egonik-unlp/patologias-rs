// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "study_type"))]
    pub struct StudyType;
}

diesel::table! {
    citologias (id) {
        id -> Int4,
        study_id -> Int4,
        microflora -> Nullable<Text>,
        celulas_pavimentosas -> Nullable<Text>,
        celulas_metaplasicas -> Nullable<Text>,
        leucocitos -> Nullable<Text>,
        hematies -> Nullable<Text>,
        celulas_endocervicales -> Nullable<Text>,
        compatible_con -> Nullable<Text>,
        sugerencia -> Nullable<Text>,
        diag_citologias -> Nullable<Text>,
    }
}

diesel::table! {
    doctors (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    histopatologias (id) {
        id -> Int4,
        study_id -> Int4,
        descripcion_macroscopica -> Nullable<Text>,
        diagnostico_histopatologico -> Nullable<Text>,
    }
}

diesel::table! {
    materials (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StudyType;

    studies (id) {
        id -> Int4,
        protocolo -> Int4,
        fecha -> Timestamp,
        tipo -> StudyType,
        #[max_length = 255]
        paciente -> Varchar,
        doctor_id -> Int4,
        material_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(citologias -> studies (study_id));
diesel::joinable!(histopatologias -> studies (study_id));
diesel::joinable!(studies -> doctors (doctor_id));
diesel::joinable!(studies -> materials (material_id));

diesel::allow_tables_to_appear_in_same_query!(
    citologias,
    doctors,
    histopatologias,
    materials,
    studies,
);

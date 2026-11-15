use diesel::{
    associations::Identifiable, deserialize::Queryable, prelude::Insertable,
    query_builder::AsChangeset, Selectable,
};
use serde::{Deserialize, Serialize};

use crate::entities::dto::{DoctorDto, MaterialsDto};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::doctors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Doctor {
    pub id: i32,
    pub name: String,
}

impl From<Doctor> for DoctorDto {
    fn from(value: Doctor) -> Self {
        let Doctor { id, name } = value;
        DoctorDto { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::materials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Material {
    id: i32,
    name: String,
}

impl From<Material> for MaterialsDto {
    fn from(value: Material) -> Self {
        let Material { id, name } = value;
        MaterialsDto { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name= crate::core::database::schema::doctors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewDoctor {
    name: String,
}

impl NewDoctor {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name= crate::core::database::schema::materials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMaterial {
    name: String,
}

impl NewMaterial {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

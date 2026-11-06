use crate::entities::model::{NewDoctor, NewMaterial};
use crate::impl_repository;
use crate::{
    core::database::connection::DbPool,
    entities::model::{Doctor, Material},
};

use crate::core::database::schema::{doctors, materials};

#[derive(Debug, Clone)]
pub struct DoctorRepository {
    pool: DbPool,
}

impl DoctorRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Clone)]
pub struct MaterialRepository {
    pool: DbPool,
}

impl MaterialRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
impl_repository!(DoctorRepository, Doctor, doctors, i32, NewDoctor);
impl_repository!(MaterialRepository, Material, materials, i32, NewMaterial);

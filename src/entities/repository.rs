use crate::entities::model::{NewDoctor, NewMaterial};
use crate::impl_repository;
use crate::{
    core::database::connection::DbPool,
    entities::model::{Doctor, Material},
};

use crate::core::database::schema::{doctors, materials};

#[derive(Debug)]
pub struct DoctorRepository {
    pool: DbPool,
}

#[derive(Debug)]
pub struct MaterialRepository {
    pool: DbPool,
}
impl_repository!(DoctorRepository, Doctor, doctors, i32, NewDoctor);
impl_repository!(MaterialRepository, Material, materials, i32, NewMaterial);

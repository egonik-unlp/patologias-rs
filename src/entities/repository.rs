use crate::entities::model::{NewDoctor, NewMaterial};
use crate::impl_repository;
use crate::{
    core::database::connection::DbPool,
    entities::model::{Doctor, Material},
};

use crate::core::database::schema::{doctors, materials};

impl_repository!(async, Doctor, doctors, i32, NewDoctor);
impl_repository!(async, Material, materials, i32, NewMaterial);

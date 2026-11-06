use crate::{
    core::database::repository::DatabaseRepository,
    entities::{
        model::{Doctor, NewDoctor, NewMaterial},
        repository::{DoctorRepository, MaterialRepository},
    },
};

#[derive(Debug, Clone)]
pub struct EntitiesService {
    doctors_repository: DoctorRepository,
    materials_repository: MaterialRepository,
}

impl EntitiesService {
    pub fn new(
        doctors_repository: DoctorRepository,
        materials_repository: MaterialRepository,
    ) -> Self {
        Self {
            doctors_repository,
            materials_repository,
        }
    }
    pub async fn create_doctor(&self, name: impl Into<String>) -> anyhow::Result<()> {
        let new_doctor = NewDoctor::new(name.into());
        self.doctors_repository.create(new_doctor).await
    }
    pub async fn create_material(&self, name: impl Into<String>) -> anyhow::Result<()> {
        let new_material = NewMaterial::new(name.into());
        self.materials_repository.create(new_material).await
    }
}

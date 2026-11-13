use crate::{
    core::database::{
        connection::interact_with_connection,
        repository::{rank_suggestions, IsField},
        schema::{doctors, materials},
    },
    entities::{
        dto::{DoctorFieldsDto, MaterialFieldsDto},
        model::{Doctor, Material, NewDoctor, NewMaterial},
    },
    impl_repository, suggestions,
};

impl_repository!(async, Doctor, doctors, i32, NewDoctor);
impl_repository!(async, Material, materials, i32, NewMaterial);

impl IsField for DoctorFieldsDto {}
impl IsField for MaterialFieldsDto {}
impl DoctorRepository {
    pub async fn get_suggestions(
        &self,
        field: DoctorFieldsDto,
        limit: usize,
    ) -> anyhow::Result<Vec<String>> {
        _ = field;
        let raw_suggestions = interact_with_connection(&self.pool, |conn| {
            suggestions!(unique, conn, doctors, doctors::name)
        })
        .await?;
        Ok(rank_suggestions(raw_suggestions.into_iter(), limit))
    }
}

impl MaterialRepository {
    pub async fn get_suggestions(
        &self,
        field: MaterialFieldsDto,
        limit: usize,
    ) -> anyhow::Result<Vec<String>> {
        _ = field;
        let raw_suggestions = interact_with_connection(&self.pool, |conn| {
            suggestions!(unique, conn, materials, materials::name)
        })
        .await?;
        Ok(rank_suggestions(raw_suggestions.into_iter(), limit))
    }
}

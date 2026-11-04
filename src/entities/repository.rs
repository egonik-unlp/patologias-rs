use crate::{
    core::database::connection::DbPool,
    entities::model::{Doctor, Material},
};

#[derive(Debug)]
pub struct EntityRepository {
    pool: DbPool,
}

impl EntityRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
    pub async fn get_doctor(&self) -> anyhow::Result<Doctor> {
        _ = &self.pool;
        todo!()
    }

    pub async fn get_material(&self) -> anyhow::Result<Material> {
        _ = &self.pool;
        todo!()
    }
}

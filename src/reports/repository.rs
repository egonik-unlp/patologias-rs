use crate::core::database::connection::DbPool;
use crate::core::database::repository::DatabaseRepository;
use crate::core::database::schema::studies;
use crate::impl_repository;
use crate::reports::model::{NewStudy, Study};

impl_repository!(ReportsRepository, Study, studies, i32, NewStudy);

pub struct ReportsRepository {
    pool: DbPool,
}

impl ReportsRepository {
    fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

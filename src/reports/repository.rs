use crate::core::database::connection::{interact_with_connection, DbPool};
use crate::core::database::repository::DatabaseRepository;
use crate::core::database::schema::{citologias, histopatologias, studies};
use crate::impl_repository;
use crate::reports::model::{
    Citologia, Histopatologia, NewCitologia, NewHistopatologia, NewStudy, Study,
};

// TODO: the parent/child (hierarchy) arm of impl_repository! is still WIP;
// use the plain CRUD arm for each table until it is finished.
// impl_repository!(
//     async,
//     Study,
//     studies,
//     i32,
//     NewStudy,
//     Study,
//     Citologia,
//     Histopatologia
// );
impl_repository!(async, Study, studies, i32, NewStudy);
impl_repository!(async, Citologia, citologias, i32, NewCitologia);
impl_repository!(async, Histopatologia, histopatologias, i32, NewHistopatologia);

pub type ReportsRepository = StudyRepository;
pub type CitologiasRepository = CitologiaRepository;
pub type HistopatologiasRepository = HistopatologiaRepository;
// impl_repository!(ReportsRepository, Study, studies, i32, NewStudy);
// impl_repository!(
//     HistopatologiasRepository,
//     Histopatologia,
//     histopatologias,
//     i32,
//     NewHistopatologia
// );
// impl_repository!(
//     CitologiasRepository,
//     Citologia,
//     citologias,
//     i32,
//     NewCitologia
// );
//
// #[derive(Debug, Clone)]
// pub struct ReportsRepository {
//     pool: DbPool,
// }
//
// impl ReportsRepository {
//     pub fn new(pool: DbPool) -> Self {
//         Self { pool }
//     }
// }
//
// #[derive(Debug, Clone)]
// pub struct HistopatologiasRepository {
//     pool: DbPool,
// }
//
// #[derive(Debug, Clone)]
// pub struct CitologiasRepository {
//     pool: DbPool,
// }
// impl CitologiasRepository {
//     pub fn new(pool: DbPool) -> Self {
//         Self { pool }
//     }
// }
//
// impl HistopatologiasRepository {
//     pub fn new(pool: DbPool) -> Self {
//         Self { pool }
//     }
// }

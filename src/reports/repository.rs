use crate::core::database::schema::{citologias, histopatologias, studies};
use crate::impl_repository;
use crate::reports::model::{
    Citologia, Histopatologia, NewCitologia, NewHistopatologia, NewStudy, Study, StudyType,
};

// Genera StudyRepository (AsyncLayerWithHierarchy), StudyDetail y NewStudyDetail.
impl_repository!(
    async, Study, studies, i32, NewStudy,
    children(study_id) {
        Citologia: citologias, NewCitologia;
        Histopatologia: histopatologias, NewHistopatologia;
    },
    kind = tipo: StudyType,
);

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

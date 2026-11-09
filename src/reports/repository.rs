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

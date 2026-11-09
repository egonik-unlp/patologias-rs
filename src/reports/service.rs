use crate::core::database::repository::AsyncLayerWithHierarchy;
use crate::reports::model::{NewStudy, Study};
use crate::reports::repository::{NewStudyDetail, StudyDetail, StudyRepository};

#[derive(Debug, Clone)]
pub struct ReportsService {
    repo: StudyRepository,
}

impl ReportsService {
    pub fn new(repo: StudyRepository) -> Self {
        Self { repo }
    }

    pub async fn create_study(
        &self,
        study: NewStudy,
        detail: NewStudyDetail,
    ) -> anyhow::Result<(Study, StudyDetail)> {
        self.repo.create(study, detail).await
    }
}

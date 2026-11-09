use crate::core::database::repository::AsyncLayerWithHierarchy;
use crate::reports::dto::FullStudyDto;
use crate::reports::model::Study;
use crate::reports::repository::{StudyDetail, StudyRepository};

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
        full_study: FullStudyDto,
    ) -> anyhow::Result<(Study, StudyDetail)> {
        let (study, detail) = full_study.into_parts();
        self.repo.create(study, detail).await
    }
    pub async fn read_all_studies(&self) -> anyhow::Result<Vec<(Study, StudyDetail)>> {
        self.repo.read_all().await
    }
}

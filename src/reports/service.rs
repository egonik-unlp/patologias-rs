use std::ops::Mul;

use crate::core::database::repository::AsyncLayerWithHierarchy;
use crate::reports::dto::{
    FullStudyDto, NewFullStudyDto, PaginatedResponseDto, ReportFieldsDto, StudyDto,
};
use crate::reports::model::Study;
use crate::reports::repository::{StudyDetail, StudyRepository};
use crate::reports::LIMIT;

#[derive(Debug, Clone)]
pub struct ReportsService {
    repo: StudyRepository,
}

// Donde tienen que desaparecer los dtos

impl ReportsService {
    pub fn new(repo: StudyRepository) -> Self {
        Self { repo }
    }

    pub async fn create_study(
        &self,
        full_study: NewFullStudyDto,
    ) -> anyhow::Result<(Study, StudyDetail)> {
        let (study, detail) = full_study.into_parts();
        self.repo.create(study, detail).await
    }

    pub async fn read_all_studies(&self) -> anyhow::Result<Vec<(Study, StudyDetail)>> {
        self.repo.read_all().await
    }
    pub async fn read_all_shallow(&self) -> anyhow::Result<Vec<StudyDto>> {
        self.repo.get_shallow().await
    }
    pub async fn get_suggestions(&self, field: ReportFieldsDto) -> anyhow::Result<Vec<String>> {
        self.repo.get_suggestions(field, 20).await
    }
    pub async fn get_report_by_id(&self, id: i32) -> anyhow::Result<FullStudyDto> {
        self.repo.read(id).await.map(FullStudyDto::from)
    }
    pub async fn delete_study(&self, id: i32) -> anyhow::Result<()> {
        self.repo.delete(id).await
    }
    pub async fn update_study(
        &self,
        update: NewFullStudyDto,
        id: i32,
    ) -> anyhow::Result<FullStudyDto> {
        let (study, child) = update.into_parts();
        self.repo
            .update(id, study, child)
            .await
            .map(FullStudyDto::from)
    }
    pub async fn read_studies_paginated(&self, page: i64) -> anyhow::Result<PaginatedResponseDto> {
        let offset = (page - 1).mul(LIMIT);
        self.repo
            .get_shallow_paginated(offset)
            .await
            .map(|(page, total)| PaginatedResponseDto::new(page, offset, total))
    }
}

use anyhow::Context;
use diesel::Connection;

use crate::core::database::{connection::interact, repository::DatabaseRepository};
use crate::reports::model::{NewStudy, NewStudyDetail, Study, StudyDetail};
use crate::reports::repository::{
    CitologiasRepository, HistopatologiasRepository, ReportsRepository,
};

#[derive(Debug, Clone)]
pub struct ReportsService {
    repo: ReportsRepository,
    histopatologias_repository: HistopatologiasRepository,
    citologias_repository: CitologiasRepository,
}

impl ReportsService {
    pub fn new(
        repo: ReportsRepository,
        histopatologias_repository: HistopatologiasRepository,
        citologias_repository: CitologiasRepository,
    ) -> Self {
        Self {
            repo,
            histopatologias_repository,
            citologias_repository,
        }
    }

    /// Inserts a study and its citología/histopatología in one transaction:
    /// if the child insert fails, the study is rolled back too.
    /// `study.tipo` is overwritten to match the detail variant.
    pub async fn create_study(
        &self,
        mut study: NewStudy,
        detail: NewStudyDetail,
    ) -> anyhow::Result<(Study, StudyDetail)> {
        study.tipo = detail.tipo();
        let studies = self.repo.clone();
        let citologias = self.citologias_repository.clone();
        let histopatologias = self.histopatologias_repository.clone();
        interact(self.repo.pool(), move |conn| {
            conn.transaction(|conn| {
                let study = studies
                    ._create(study, conn, None)
                    .context("creating study")?;
                let detail = match detail {
                    NewStudyDetail::Citologia(mut citologia) => {
                        citologia.study_id = Some(study.id());
                        StudyDetail::Citologia(
                            citologias
                                ._create(citologia, conn, None)
                                .context("creating citologia")?,
                        )
                    }
                    NewStudyDetail::Histopatologia(mut histopatologia) => {
                        histopatologia.study_id = Some(study.id());
                        StudyDetail::Histopatologia(
                            histopatologias
                                ._create(histopatologia, conn, None)
                                .context("creating histopatologia")?,
                        )
                    }
                };
                Ok((study, detail))
            })
        })
        .await
    }

    // pub async fn get_all_reports(&self) -> anyhow::Result<Vec<String>> {
    //     self.repo
    //         .read_all()
    //         .await
    //         .map(|inner| inner.iter().map(|item| format!("{:?}", inner)).collect())
    // }
}

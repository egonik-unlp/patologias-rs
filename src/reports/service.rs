use crate::core::database::repository::DatabaseRepository;
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
    pub async fn get_all_reports(&self) -> anyhow::Result<Vec<String>> {
        self.repo
            .read_all()
            .await
            .map(|inner| inner.iter().map(|item| format!("{:?}", inner)).collect())
    }
}

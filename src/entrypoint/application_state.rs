use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::{
    core::database::connection::DbPool,
    entities::{
        repository::{DoctorRepository, MaterialRepository},
        service::EntitiesService,
    },
    reports::{
        repository::{CitologiasRepository, HistopatologiasRepository, ReportsRepository},
        service::ReportsService,
    },
};

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub entities_service: EntitiesService,
    pub reports_service: ReportsService,
}

impl AppState {
    pub fn new(pool: DbPool, leptos_options: LeptosOptions) -> Self {
        let doctors_repository = DoctorRepository::new(pool.clone());
        let material_repository = MaterialRepository::new(pool.clone());
        let histopatologias_repository = HistopatologiasRepository::new(pool.clone());
        let citologias_repository = CitologiasRepository::new(pool.clone());
        let reports_repository = ReportsRepository::new(pool.clone());
        let entities_service = EntitiesService::new(doctors_repository, material_repository);
        let reports_service = ReportsService::new(
            reports_repository,
            histopatologias_repository,
            citologias_repository,
        );
        Self {
            leptos_options,
            entities_service,
            reports_service,
        }
    }
}

use axum::extract::FromRef;
use leptos::config::LeptosOptions;

use crate::{
    core::database::connection::DbPool,
    entities::{
        repository::{DoctorRepository, MaterialRepository},
        service::EntitiesService,
    },
    reports::{repository::StudyRepository, service::ReportsService},
    users::{
        repository::{SessionRepository, UserRepository},
        service::UserService,
    },
};

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub entities_service: EntitiesService,
    pub reports_service: ReportsService,
    pub users_service: UserService,
}

impl AppState {
    pub fn new(pool: DbPool, leptos_options: LeptosOptions) -> Self {
        let doctors_repository = DoctorRepository::new(pool.clone());
        let material_repository = MaterialRepository::new(pool.clone());
        let studies_repository = StudyRepository::new(pool.clone());
        let users_repository = UserRepository::new(pool.clone());
        let sessions_repository = SessionRepository::new(pool.clone());
        let users_service = UserService::new(users_repository, sessions_repository);
        let entities_service = EntitiesService::new(doctors_repository, material_repository);
        let reports_service = ReportsService::new(studies_repository);
        Self {
            leptos_options,
            entities_service,
            reports_service,
            users_service,
        }
    }
}

use std::future::Future;

use leptos::prelude::*;

use crate::{
    entities::dto::{DoctorFieldsDto, MaterialFieldsDto},
    frontend::suggestions::Suggestions,
};

#[server]
pub async fn get_suggestions_for_doctors(
    field: DoctorFieldsDto,
) -> Result<Vec<String>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("cant get appstate"))?;
    ctx.entities_service
        .get_suggestions_doctors(field)
        .await
        .map_err(ServerFnError::new)
}
#[server]
pub async fn get_suggestions_for_materials(
    field: MaterialFieldsDto,
) -> Result<Vec<String>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let ctx = use_context::<AppState>().ok_or_else(|| ServerFnError::new("cant get appstate"))?;
    ctx.entities_service
        .get_suggestions_materials(field)
        .await
        .map_err(ServerFnError::new)
}

impl Suggestions for MaterialFieldsDto {
    fn get_suggestions(
        &self,
    ) -> impl Future<Output = Result<Vec<String>, ServerFnError>> + Send + 'static {
        get_suggestions_for_materials(self.clone())
    }
}

impl Suggestions for DoctorFieldsDto {
    fn get_suggestions(
        &self,
    ) -> impl Future<Output = Result<Vec<String>, ServerFnError>> + Send + 'static {
        get_suggestions_for_doctors(self.clone())
    }
}

//! Report UI, split by page. Route components are re-exported here so callers
//! keep importing from `reports::component`.
mod detail;
mod form;
mod list;
mod sections;

pub use detail::Report;
pub use form::{CreateReport, UpdateReport};
pub use list::Reports;

use crate::{
    frontend::icons::{IconCitologia, IconHistopatologia},
    reports::dto::StudyTypeDto,
};
use leptos::prelude::*;

/// Icon that tells the two study types apart at a glance (cell vs. slide).
#[component]
fn StudyTypeIcon(tipo: StudyTypeDto, #[prop(into, optional)] class: String) -> impl IntoView {
    match tipo {
        StudyTypeDto::Citologia => view! { <IconCitologia class /> }.into_any(),
        StudyTypeDto::Histopatologia => view! { <IconHistopatologia class /> }.into_any(),
    }
}
